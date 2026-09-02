use fennara_project_identity::ProjectRoot;
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex, MutexGuard},
};

pub(crate) const DEFAULT_MAX_RUN_SECONDS: u64 = 900;
pub(crate) const MAX_RUN_SECONDS: u64 = 86_400;
pub(crate) const INACTIVITY_TIMEOUT_MS: u64 = 120_000;
pub(crate) const HEARTBEAT_INTERVAL_MS: u64 = 30_000;
pub(crate) const STARTUP_CLAIM_TIMEOUT_MS: u64 = 30_000;
pub(crate) const BUSY_RETRY_AFTER_MS: u64 = 2_000;
const MAX_OWNER_OPERATION_MS: u64 = 120_000;

#[derive(Clone)]
pub(crate) struct RuntimeSlot {
    inner: Arc<Mutex<RuntimeSlotInner>>,
}

struct RuntimeSlotInner {
    state: SlotState,
    next_token: u64,
}

enum SlotState {
    Free,
    Starting {
        claim_token: u64,
        owner: ProjectRoot,
        startup_deadline_ms: u64,
    },
    Running {
        claim_token: u64,
        session_id: String,
        owner: ProjectRoot,
        lease: RuntimeLease,
    },
    Cleaning {
        cleanup_token: u64,
        session_id: String,
        owner: ProjectRoot,
    },
    ShuttingDown,
}

pub(crate) struct StartClaim {
    slot: RuntimeSlot,
    claim_token: u64,
    phase: StartClaimPhase,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StartClaimPhase {
    BeforeProcess,
    ProcessMayExist,
    Resolved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SlotBusy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SlotTransitionError {
    ClaimExpired,
    ClaimLost,
    InvalidLease(&'static str),
}

impl fmt::Display for SlotTransitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClaimExpired => formatter.write_str("runtime start claim expired"),
            Self::ClaimLost => formatter.write_str("runtime start claim is no longer active"),
            Self::InvalidLease(message) => formatter.write_str(message),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum SlotObservation {
    Free,
    Busy,
    Owned(OwnedRuntime),
    NotOwnedOrFound,
}

#[derive(Clone, Debug)]
pub(crate) struct OwnedRuntime {
    pub(crate) session_id: String,
    pub(crate) lease: LeaseSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LeaseExpiry {
    Absolute,
    Inactivity,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuntimeExpiration {
    pub(crate) session_id: String,
    pub(crate) reason: LeaseExpiry,
}

#[derive(Clone, Debug)]
pub(crate) struct RuntimeLease {
    absolute_deadline_ms: u64,
    last_activity_ms: u64,
    next_operation_token: u64,
    active_operations: HashMap<u64, tokio::time::Instant>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LeaseSnapshot {
    pub(crate) absolute_deadline_ms: u64,
    pub(crate) absolute_remaining_seconds: u64,
    pub(crate) inactivity_deadline_ms: Option<u64>,
    pub(crate) inactivity_remaining_seconds: Option<u64>,
    pub(crate) heartbeat_interval_ms: u64,
}

pub(crate) struct OwnerOperation {
    slot: RuntimeSlot,
    claim_token: u64,
    session_id: String,
    operation_token: u64,
    deadline: tokio::time::Instant,
    finished: bool,
}

struct OperationAnchor {
    now_ms: u64,
    monotonic_now: tokio::time::Instant,
}

pub(crate) struct CleanupClaim {
    slot: RuntimeSlot,
    cleanup_token: u64,
    session_id: String,
    reason: Option<LeaseExpiry>,
}

impl RuntimeSlot {
    pub(crate) fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(RuntimeSlotInner {
                state: SlotState::Free,
                next_token: 1,
            })),
        }
    }

    pub(crate) fn try_claim(
        &self,
        owner: ProjectRoot,
        now_ms: u64,
    ) -> Result<StartClaim, SlotBusy> {
        let mut inner = self.lock();
        if !matches!(&inner.state, SlotState::Free) {
            return Err(SlotBusy);
        }

        let claim_token = Self::take_next_token(&mut inner);
        inner.state = SlotState::Starting {
            claim_token,
            owner,
            startup_deadline_ms: now_ms.saturating_add(STARTUP_CLAIM_TIMEOUT_MS),
        };
        Ok(StartClaim {
            slot: self.clone(),
            claim_token,
            phase: StartClaimPhase::BeforeProcess,
        })
    }

    pub(crate) fn is_occupied_now(&self) -> bool {
        matches!(
            &self.lock().state,
            SlotState::Starting { .. } | SlotState::Running { .. } | SlotState::Cleaning { .. }
        )
    }

    pub(crate) async fn is_occupied(&self) -> bool {
        self.is_occupied_now()
    }

    #[cfg(test)]
    pub(crate) fn observe(
        &self,
        requester: &ProjectRoot,
        requested_session_id: Option<&str>,
        now_ms: u64,
    ) -> SlotObservation {
        let inner = self.lock();
        observation(&inner.state, requester, requested_session_id, now_ms)
    }

    pub(crate) fn renew_and_observe(
        &self,
        requester: &ProjectRoot,
        requested_session_id: Option<&str>,
        now_ms: u64,
    ) -> SlotObservation {
        let mut inner = self.lock();
        if let SlotState::Running {
            session_id,
            owner,
            lease,
            ..
        } = &mut inner.state
            && owner.same_project(requester)
            && requested_session_id.is_none_or(|requested| requested == session_id)
            && lease.expiry(now_ms).is_none()
        {
            lease.renew(now_ms);
        }
        observation(&inner.state, requester, requested_session_id, now_ms)
    }

    pub(crate) fn begin_owner_operation(
        &self,
        requester: &ProjectRoot,
        session_id: &str,
        operation_timeout_ms: u64,
    ) -> Result<OwnerOperation, SlotObservation> {
        self.begin_owner_operation_with_anchor(requester, session_id, operation_timeout_ms, || {
            let admitted_at = tokio::time::Instant::now();
            let now_ms = super::util::unix_millis().min(u128::from(u64::MAX)) as u64;
            OperationAnchor {
                now_ms,
                monotonic_now: admitted_at,
            }
        })
    }

    fn begin_owner_operation_with_anchor(
        &self,
        requester: &ProjectRoot,
        session_id: &str,
        operation_timeout_ms: u64,
        anchor: impl FnOnce() -> OperationAnchor,
    ) -> Result<OwnerOperation, SlotObservation> {
        let mut inner = self.lock();
        let SlotState::Running {
            claim_token,
            session_id: running_session_id,
            owner,
            lease,
        } = &mut inner.state
        else {
            return Err(SlotObservation::NotOwnedOrFound);
        };
        if running_session_id != session_id || !owner.same_project(requester) {
            return Err(SlotObservation::NotOwnedOrFound);
        }
        // Establish both clocks only after this operation owns the slot lock so
        // lock contention cannot consume the admitted operation lifetime.
        let anchor = anchor();
        if lease
            .expiry_at(anchor.now_ms, anchor.monotonic_now)
            .is_some()
        {
            return Err(SlotObservation::NotOwnedOrFound);
        }

        let (operation_token, deadline) =
            lease.begin_operation(anchor.monotonic_now, operation_timeout_ms);
        Ok(OwnerOperation {
            slot: self.clone(),
            claim_token: *claim_token,
            session_id: running_session_id.clone(),
            operation_token,
            deadline,
            finished: false,
        })
    }

    #[cfg(test)]
    fn begin_owner_operation_at(
        &self,
        requester: &ProjectRoot,
        session_id: &str,
        now_ms: u64,
        operation_timeout_ms: u64,
    ) -> Result<OwnerOperation, SlotObservation> {
        self.begin_owner_operation_with_anchor(requester, session_id, operation_timeout_ms, || {
            let monotonic_now = tokio::time::Instant::now();
            OperationAnchor {
                now_ms,
                monotonic_now,
            }
        })
    }

    #[cfg(test)]
    fn running_expiration(&self, now_ms: u64) -> Option<RuntimeExpiration> {
        let inner = self.lock();
        let SlotState::Running {
            session_id, lease, ..
        } = &inner.state
        else {
            return None;
        };
        lease.expiry(now_ms).map(|reason| RuntimeExpiration {
            session_id: session_id.clone(),
            reason,
        })
    }

    pub(crate) fn claim_expired(&self, now_ms: u64) -> Option<CleanupClaim> {
        let mut inner = self.lock();
        let (session_id, owner, reason) = match &inner.state {
            SlotState::Running {
                session_id,
                owner,
                lease,
                ..
            } => (session_id.clone(), owner.clone(), lease.expiry(now_ms)?),
            _ => return None,
        };
        let cleanup_token = Self::take_next_token(&mut inner);
        inner.state = SlotState::Cleaning {
            cleanup_token,
            session_id: session_id.clone(),
            owner,
        };
        Some(CleanupClaim {
            slot: self.clone(),
            cleanup_token,
            session_id,
            reason: Some(reason),
        })
    }

    pub(crate) fn begin_owner_cleanup(
        &self,
        requester: &ProjectRoot,
        session_id: &str,
        now_ms: u64,
    ) -> Result<CleanupClaim, SlotObservation> {
        let mut inner = self.lock();
        let SlotState::Running {
            session_id: current,
            owner,
            lease,
            ..
        } = &inner.state
        else {
            return Err(SlotObservation::NotOwnedOrFound);
        };
        if current != session_id || !owner.same_project(requester) || lease.expiry(now_ms).is_some()
        {
            return Err(SlotObservation::NotOwnedOrFound);
        }
        let owner = owner.clone();
        Ok(self.transition_to_cleanup(&mut inner, session_id, owner, None))
    }

    pub(crate) fn claim_finished(&self, session_id: &str) -> Option<CleanupClaim> {
        let mut inner = self.lock();
        let owner = match &inner.state {
            SlotState::Running {
                session_id: current,
                owner,
                ..
            } if current == session_id => owner.clone(),
            _ => return None,
        };
        Some(self.transition_to_cleanup(&mut inner, session_id, owner, None))
    }

    pub(crate) fn begin_shutdown(&self) -> bool {
        let mut inner = self.lock();
        if matches!(&inner.state, SlotState::Free) {
            inner.state = SlotState::ShuttingDown;
            true
        } else {
            false
        }
    }

    fn lock(&self) -> MutexGuard<'_, RuntimeSlotInner> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn release_start_claim(&self, claim_token: u64) -> bool {
        let mut inner = self.lock();
        if matches!(&inner.state, SlotState::Starting { claim_token: current, .. } if *current == claim_token)
        {
            inner.state = SlotState::Free;
            true
        } else {
            false
        }
    }

    fn release_cleanup_claim(&self, cleanup_token: u64, session_id: &str) -> bool {
        let mut inner = self.lock();
        if matches!(
            &inner.state,
            SlotState::Cleaning {
                cleanup_token: current,
                session_id: current_session,
                ..
            } if *current == cleanup_token && current_session == session_id
        ) {
            inner.state = SlotState::Free;
            true
        } else {
            false
        }
    }

    fn transition_to_cleanup(
        &self,
        inner: &mut RuntimeSlotInner,
        session_id: &str,
        owner: ProjectRoot,
        reason: Option<LeaseExpiry>,
    ) -> CleanupClaim {
        let cleanup_token = Self::take_next_token(inner);
        inner.state = SlotState::Cleaning {
            cleanup_token,
            session_id: session_id.to_string(),
            owner,
        };
        CleanupClaim {
            slot: self.clone(),
            cleanup_token,
            session_id: session_id.to_string(),
            reason,
        }
    }

    fn take_next_token(inner: &mut RuntimeSlotInner) -> u64 {
        let token = inner.next_token;
        inner.next_token = inner.next_token.wrapping_add(1);
        token
    }
}

impl Default for RuntimeSlot {
    fn default() -> Self {
        Self::new()
    }
}

impl StartClaim {
    /// Records the point after which dropping this claim must fail closed.
    ///
    /// The caller must invoke this immediately after spawning the child. Once a
    /// process may exist, only `release_after_cleanup` can make the slot free.
    pub(crate) fn mark_process_spawned(&mut self) {
        if self.phase == StartClaimPhase::BeforeProcess {
            self.phase = StartClaimPhase::ProcessMayExist;
        }
    }

    pub(crate) fn commit(
        &mut self,
        session_id: String,
        max_run_seconds: u64,
        now_ms: u64,
    ) -> Result<LeaseSnapshot, SlotTransitionError> {
        if self.phase != StartClaimPhase::ProcessMayExist {
            return Err(SlotTransitionError::ClaimLost);
        }
        let mut inner = self.slot.lock();
        let SlotState::Starting {
            claim_token,
            owner,
            startup_deadline_ms,
        } = &inner.state
        else {
            return Err(SlotTransitionError::ClaimLost);
        };
        if *claim_token != self.claim_token {
            return Err(SlotTransitionError::ClaimLost);
        }
        if now_ms >= *startup_deadline_ms {
            return Err(SlotTransitionError::ClaimExpired);
        }

        let lease = RuntimeLease::new(now_ms, Some(max_run_seconds))
            .map_err(SlotTransitionError::InvalidLease)?;
        let snapshot = lease.snapshot(now_ms);
        let owner = owner.clone();
        inner.state = SlotState::Running {
            claim_token: self.claim_token,
            session_id,
            owner,
            lease,
        };
        self.phase = StartClaimPhase::Resolved;
        Ok(snapshot)
    }

    /// Releases the claim only after any spawned process has been killed and
    /// reaped. Consuming the claim prevents a second release attempt.
    pub(crate) fn release_after_cleanup(mut self) -> bool {
        let released = self.slot.release_start_claim(self.claim_token);
        self.phase = StartClaimPhase::Resolved;
        released
    }
}

impl Drop for StartClaim {
    fn drop(&mut self) {
        if self.phase == StartClaimPhase::BeforeProcess {
            self.slot.release_start_claim(self.claim_token);
        }
    }
}

impl RuntimeLease {
    pub(crate) fn new(started_ms: u64, max_run_seconds: Option<u64>) -> Result<Self, &'static str> {
        let max_run_seconds = validate_max_run_seconds(max_run_seconds)?;
        Ok(Self {
            absolute_deadline_ms: started_ms.saturating_add(max_run_seconds.saturating_mul(1_000)),
            last_activity_ms: started_ms,
            next_operation_token: 1,
            active_operations: HashMap::new(),
        })
    }

    pub(crate) fn renew(&mut self, now_ms: u64) {
        self.last_activity_ms = self.last_activity_ms.max(now_ms);
    }

    pub(crate) fn begin_operation(
        &mut self,
        monotonic_now: tokio::time::Instant,
        operation_timeout_ms: u64,
    ) -> (u64, tokio::time::Instant) {
        let token = self.next_operation_token;
        self.next_operation_token = self.next_operation_token.saturating_add(1);
        let deadline = monotonic_now
            + std::time::Duration::from_millis(
                operation_timeout_ms.clamp(1, MAX_OWNER_OPERATION_MS),
            );
        self.active_operations.insert(token, deadline);
        (token, deadline)
    }

    pub(crate) fn finish_operation(&mut self, token: u64, now_ms: u64) {
        self.finish_operation_at(token, now_ms, tokio::time::Instant::now());
    }

    fn finish_operation_at(
        &mut self,
        token: u64,
        now_ms: u64,
        monotonic_now: tokio::time::Instant,
    ) {
        if self
            .active_operations
            .remove(&token)
            .is_some_and(|deadline| monotonic_now < deadline && now_ms < self.absolute_deadline_ms)
        {
            self.renew(now_ms);
        }
    }

    fn cancel_operation(&mut self, token: u64) {
        self.active_operations.remove(&token);
    }

    pub(crate) fn expiry(&self, now_ms: u64) -> Option<LeaseExpiry> {
        self.expiry_at(now_ms, tokio::time::Instant::now())
    }

    fn expiry_at(&self, now_ms: u64, monotonic_now: tokio::time::Instant) -> Option<LeaseExpiry> {
        if now_ms >= self.absolute_deadline_ms {
            return Some(LeaseExpiry::Absolute);
        }
        if self
            .active_operations
            .values()
            .any(|deadline| monotonic_now < *deadline)
        {
            return None;
        }
        (now_ms >= self.inactivity_deadline_ms()).then_some(LeaseExpiry::Inactivity)
    }

    pub(crate) fn snapshot(&self, now_ms: u64) -> LeaseSnapshot {
        self.snapshot_at(now_ms, tokio::time::Instant::now())
    }

    fn snapshot_at(&self, now_ms: u64, monotonic_now: tokio::time::Instant) -> LeaseSnapshot {
        let operation_is_active = self
            .active_operations
            .values()
            .any(|deadline| monotonic_now < *deadline);
        let inactivity_deadline_ms = (!operation_is_active).then(|| self.inactivity_deadline_ms());
        LeaseSnapshot {
            absolute_deadline_ms: self.absolute_deadline_ms,
            absolute_remaining_seconds: remaining_seconds(self.absolute_deadline_ms, now_ms),
            inactivity_deadline_ms,
            inactivity_remaining_seconds: inactivity_deadline_ms
                .map(|deadline| remaining_seconds(deadline, now_ms)),
            heartbeat_interval_ms: HEARTBEAT_INTERVAL_MS,
        }
    }

    fn inactivity_deadline_ms(&self) -> u64 {
        self.last_activity_ms.saturating_add(INACTIVITY_TIMEOUT_MS)
    }
}

pub(crate) fn validate_max_run_seconds(max_run_seconds: Option<u64>) -> Result<u64, &'static str> {
    let max_run_seconds = max_run_seconds.unwrap_or(DEFAULT_MAX_RUN_SECONDS);
    if !(1..=MAX_RUN_SECONDS).contains(&max_run_seconds) {
        return Err("max_run_seconds must be an integer from 1 through 86400");
    }
    Ok(max_run_seconds)
}

impl OwnerOperation {
    pub(crate) fn deadline(&self) -> tokio::time::Instant {
        self.deadline
    }

    pub(crate) fn finish(mut self, now_ms: u64) {
        self.finish_inner(Some(now_ms));
        self.finished = true;
    }

    fn finish_inner(&self, now_ms: Option<u64>) {
        let mut inner = self.slot.lock();
        let SlotState::Running {
            claim_token,
            session_id,
            lease,
            ..
        } = &mut inner.state
        else {
            return;
        };
        if *claim_token != self.claim_token || session_id != &self.session_id {
            return;
        }
        if let Some(now_ms) = now_ms {
            lease.finish_operation(self.operation_token, now_ms);
        } else {
            lease.cancel_operation(self.operation_token);
        }
    }
}

impl Drop for OwnerOperation {
    fn drop(&mut self) {
        if !self.finished {
            self.finish_inner(None);
        }
    }
}

impl CleanupClaim {
    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    pub(crate) fn reason(&self) -> Option<LeaseExpiry> {
        self.reason
    }

    /// Makes the slot available only after the Adapter has confirmed that the
    /// corresponding child is no longer running.
    pub(crate) fn release_after_reap(self) -> bool {
        self.slot
            .release_cleanup_claim(self.cleanup_token, &self.session_id)
    }
}

fn observation(
    state: &SlotState,
    requester: &ProjectRoot,
    requested_session_id: Option<&str>,
    now_ms: u64,
) -> SlotObservation {
    match state {
        SlotState::Free => SlotObservation::Free,
        SlotState::Starting { .. } | SlotState::ShuttingDown => {
            if requested_session_id.is_some() {
                SlotObservation::NotOwnedOrFound
            } else {
                SlotObservation::Busy
            }
        }
        SlotState::Cleaning {
            session_id, owner, ..
        } => {
            if requested_session_id.is_none()
                || (owner.same_project(requester)
                    && requested_session_id == Some(session_id.as_str()))
            {
                SlotObservation::Busy
            } else {
                SlotObservation::NotOwnedOrFound
            }
        }
        SlotState::Running {
            session_id,
            owner,
            lease,
            ..
        } => {
            let requested_session_matches =
                requested_session_id.is_none_or(|requested| requested == session_id);
            if lease.expiry(now_ms).is_some() {
                if requested_session_id.is_some() {
                    SlotObservation::NotOwnedOrFound
                } else {
                    SlotObservation::Busy
                }
            } else if owner.same_project(requester) && requested_session_matches {
                SlotObservation::Owned(OwnedRuntime {
                    session_id: session_id.clone(),
                    lease: lease.snapshot(now_ms),
                })
            } else if requested_session_id.is_some() {
                SlotObservation::NotOwnedOrFound
            } else {
                SlotObservation::Busy
            }
        }
    }
}

fn remaining_seconds(deadline_ms: u64, now_ms: u64) -> u64 {
    deadline_ms.saturating_sub(now_ms).div_ceil(1_000)
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_MAX_RUN_SECONDS, HEARTBEAT_INTERVAL_MS, LeaseExpiry, MAX_RUN_SECONDS, RuntimeLease,
        RuntimeSlot, STARTUP_CLAIM_TIMEOUT_MS, SlotObservation, SlotTransitionError,
    };
    use fennara_project_identity::ProjectRoot;
    use std::{
        fs,
        ops::Deref,
        path::PathBuf,
        sync::{
            Arc,
            atomic::{AtomicU64, Ordering},
        },
    };

    static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(1);

    struct ProjectFixture {
        root: PathBuf,
        identity: ProjectRoot,
    }

    impl ProjectFixture {
        fn owned_identity(&self) -> ProjectRoot {
            self.identity.clone()
        }
    }

    impl Deref for ProjectFixture {
        type Target = ProjectRoot;

        fn deref(&self) -> &Self::Target {
            &self.identity
        }
    }

    impl Drop for ProjectFixture {
        fn drop(&mut self) {
            if let Err(error) = fs::remove_dir_all(&self.root)
                && error.kind() != std::io::ErrorKind::NotFound
            {
                eprintln!(
                    "Failed to remove Runtime Slot test fixture {}: {error}",
                    self.root.display()
                );
            }
        }
    }

    fn project(name: &str) -> ProjectFixture {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "fennara-runtime-slot-{}-{sequence}-{name}",
            std::process::id(),
        ));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("project.godot"), b"[application]\n").unwrap();
        let identity = ProjectRoot::resolve_absolute(root.as_os_str()).unwrap();
        ProjectFixture { root, identity }
    }

    fn commit_runtime(
        slot: &RuntimeSlot,
        owner: &ProjectRoot,
        session_id: &str,
        claim_ms: u64,
        commit_ms: u64,
        max_run_seconds: u64,
    ) -> super::LeaseSnapshot {
        let mut claim = slot.try_claim(owner.clone(), claim_ms).unwrap();
        claim.mark_process_spawned();
        claim
            .commit(session_id.to_string(), max_run_seconds, commit_ms)
            .unwrap()
    }

    #[test]
    fn only_one_concurrent_start_claim_can_own_the_machine_slot() {
        let slot = Arc::new(RuntimeSlot::new());
        let owner_a = project("atomic-a");
        let owner_b = project("atomic-b");
        let barrier = Arc::new(std::sync::Barrier::new(3));

        let contenders = [owner_a, owner_b].map(|owner| {
            let slot = Arc::clone(&slot);
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                let claim = slot.try_claim(owner.owned_identity(), 1_000);
                barrier.wait();
                claim.is_ok()
            })
        });
        barrier.wait();
        barrier.wait();

        assert_eq!(
            contenders
                .into_iter()
                .map(|thread| usize::from(thread.join().unwrap()))
                .sum::<usize>(),
            1
        );
    }

    #[test]
    fn dropping_an_uncommitted_start_claim_releases_the_slot() {
        let slot = RuntimeSlot::new();
        let owner_a = project("drop-a");
        let owner_b = project("drop-b");

        let claim = slot.try_claim(owner_a.owned_identity(), 1_000).unwrap();
        assert!(slot.is_occupied_now());
        drop(claim);

        assert!(slot.try_claim(owner_b.owned_identity(), 2_000).is_ok());
    }

    #[test]
    fn dropping_a_claim_after_spawn_keeps_the_slot_fail_closed() {
        let slot = RuntimeSlot::new();
        let owner_a = project("spawn-drop-a");
        let owner_b = project("spawn-drop-b");

        let mut claim = slot.try_claim(owner_a.owned_identity(), 1_000).unwrap();
        claim.mark_process_spawned();
        drop(claim);

        assert!(slot.is_occupied_now());
        assert!(slot.try_claim(owner_b.owned_identity(), 2_000).is_err());
    }

    #[test]
    fn an_expired_start_claim_stays_busy_until_its_process_is_reaped() {
        let slot = RuntimeSlot::new();
        let owner_a = project("expired-claim-a");
        let owner_b = project("expired-claim-b");
        let mut claim = slot.try_claim(owner_a.owned_identity(), 1_000).unwrap();
        claim.mark_process_spawned();

        assert!(
            slot.try_claim(owner_b.clone(), 1_000 + STARTUP_CLAIM_TIMEOUT_MS)
                .is_err()
        );
        assert_eq!(
            claim.commit(
                "stale-runtime".to_string(),
                DEFAULT_MAX_RUN_SECONDS,
                1_000 + STARTUP_CLAIM_TIMEOUT_MS,
            ),
            Err(SlotTransitionError::ClaimExpired)
        );
        assert!(slot.is_occupied_now());

        assert!(claim.release_after_cleanup());
        let mut replacement = slot.try_claim(owner_b.owned_identity(), 31_001).unwrap();
        replacement.mark_process_spawned();
        replacement
            .commit(
                "replacement-runtime".to_string(),
                DEFAULT_MAX_RUN_SECONDS,
                31_002,
            )
            .unwrap();
        assert!(slot.is_occupied_now());
    }

    #[test]
    fn dropped_cleanup_authority_fails_closed_until_reap_is_confirmed() {
        let slot = RuntimeSlot::new();
        let owner = project("stale-release");
        commit_runtime(
            &slot,
            &owner,
            "current-runtime",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );

        drop(slot.claim_finished("current-runtime").unwrap());
        assert!(slot.is_occupied_now());

        // A dropped cleanup authority cannot be recreated, so the state stays
        // fail-closed instead of guessing that the child has exited.
        assert!(slot.claim_finished("current-runtime").is_none());
    }

    #[test]
    fn cleanup_authority_releases_only_after_reap_confirmation() {
        let slot = RuntimeSlot::new();
        let owner = project("cleanup-release");
        commit_runtime(
            &slot,
            &owner,
            "current-runtime",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );

        let cleanup = slot.claim_finished("current-runtime").unwrap();
        assert!(slot.is_occupied_now());
        assert!(cleanup.release_after_reap());
        assert!(!slot.is_occupied_now());
    }

    #[test]
    fn running_status_is_detailed_only_for_the_owner() {
        let slot = RuntimeSlot::new();
        let owner = project("status-owner");
        let outsider = project("status-outsider");
        commit_runtime(&slot, &owner, "runtime-owned", 1_000, 1_500, 4_500);

        assert!(matches!(
            slot.observe(&outsider, None, 2_000),
            SlotObservation::Busy
        ));
        assert!(matches!(
            slot.observe(&outsider, Some("runtime-owned"), 2_000),
            SlotObservation::NotOwnedOrFound
        ));
        assert!(matches!(
            slot.observe(&owner, None, 2_000),
            SlotObservation::Owned(ref running) if running.session_id == "runtime-owned"
        ));
    }

    #[test]
    fn cleaning_status_stays_busy_for_the_named_owner_without_leaking_to_outsiders() {
        let slot = RuntimeSlot::new();
        let owner = project("cleaning-status-owner");
        let outsider = project("cleaning-status-outsider");
        commit_runtime(
            &slot,
            &owner,
            "runtime-cleaning",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );
        let cleanup = slot.claim_finished("runtime-cleaning").unwrap();

        assert!(matches!(
            slot.renew_and_observe(&owner, Some("runtime-cleaning"), 2_000),
            SlotObservation::Busy
        ));
        assert!(matches!(
            slot.renew_and_observe(&owner, Some("another-runtime"), 2_000),
            SlotObservation::NotOwnedOrFound
        ));
        assert!(matches!(
            slot.renew_and_observe(&outsider, Some("runtime-cleaning"), 2_000),
            SlotObservation::NotOwnedOrFound
        ));
        assert!(matches!(
            slot.renew_and_observe(&outsider, None, 2_000),
            SlotObservation::Busy
        ));

        assert!(cleanup.release_after_reap());
    }

    #[test]
    fn running_lease_begins_at_commit_after_startup_finishes() {
        let slot = RuntimeSlot::new();
        let owner = project("lease-at-commit");

        let snapshot = commit_runtime(&slot, &owner, "short-run", 1_000, 30_999, 1);

        assert_eq!(snapshot.absolute_deadline_ms, 31_999);
        assert_eq!(snapshot.absolute_remaining_seconds, 1);
        assert_eq!(snapshot.inactivity_remaining_seconds, Some(120));
    }

    #[test]
    fn shutdown_seal_atomically_blocks_new_start_claims() {
        let slot = RuntimeSlot::new();
        let owner = project("shutdown-seal");

        assert!(slot.begin_shutdown());
        assert!(!slot.is_occupied_now());
        assert!(slot.try_claim(owner.clone(), 1_000).is_err());
        assert!(matches!(
            slot.observe(&owner, None, 1_000),
            SlotObservation::Busy
        ));
        assert!(!slot.begin_shutdown());
    }

    #[test]
    fn shutdown_seal_loses_to_an_existing_start_claim() {
        let slot = RuntimeSlot::new();
        let owner = project("shutdown-busy");
        let _claim = slot.try_claim(owner.owned_identity(), 1_000).unwrap();

        assert!(!slot.begin_shutdown());
    }

    #[test]
    fn owner_activity_renews_inactivity_but_not_the_absolute_deadline() {
        let mut lease = RuntimeLease::new(10_000, Some(4_500)).unwrap();
        let absolute_deadline = lease.snapshot(10_000).absolute_deadline_ms;

        lease.renew(70_000);
        let renewed = lease.snapshot(70_000);

        assert_eq!(renewed.absolute_deadline_ms, absolute_deadline);
        assert_eq!(renewed.inactivity_deadline_ms, Some(190_000));
        assert_eq!(lease.expiry(189_999), None);
        assert_eq!(lease.expiry(190_000), Some(LeaseExpiry::Inactivity));
    }

    #[test]
    fn bounded_owner_operation_suspends_only_inactivity_expiry() {
        let mut lease = RuntimeLease::new(10_000, Some(DEFAULT_MAX_RUN_SECONDS)).unwrap();
        let monotonic_start = tokio::time::Instant::now();
        lease.begin_operation(monotonic_start, 60_000);

        assert_eq!(lease.expiry_at(130_000, monotonic_start), None);
        assert_eq!(
            lease.expiry_at(
                159_999,
                monotonic_start + std::time::Duration::from_millis(59_999)
            ),
            None
        );
        assert_eq!(
            lease.expiry_at(
                160_000,
                monotonic_start + std::time::Duration::from_millis(60_000)
            ),
            Some(LeaseExpiry::Inactivity)
        );

        let mut absolute = RuntimeLease::new(10_000, Some(1)).unwrap();
        absolute.begin_operation(monotonic_start, 60_000);
        assert_eq!(
            absolute.expiry_at(11_000, monotonic_start),
            Some(LeaseExpiry::Absolute)
        );
    }

    #[test]
    fn dropping_an_owner_operation_removes_its_inactivity_suspension() {
        let slot = RuntimeSlot::new();
        let owner = project("operation-drop");
        commit_runtime(
            &slot,
            &owner,
            "runtime-operation",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );

        let operation = slot
            .begin_owner_operation_at(&owner, "runtime-operation", 100_000, 60_000)
            .unwrap();
        assert_eq!(slot.running_expiration(130_000), None);
        drop(operation);
        assert_eq!(
            slot.running_expiration(220_000),
            Some(super::RuntimeExpiration {
                session_id: "runtime-operation".to_string(),
                reason: LeaseExpiry::Inactivity,
            })
        );
    }

    #[test]
    fn cancelled_owner_operation_does_not_renew_inactivity_at_admission() {
        let slot = RuntimeSlot::new();
        let owner = project("operation-cancel-no-renew");
        commit_runtime(
            &slot,
            &owner,
            "runtime-operation-cancel",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );

        let operation = slot
            .begin_owner_operation_at(&owner, "runtime-operation-cancel", 100_000, 60_000)
            .unwrap();
        drop(operation);

        assert_eq!(
            slot.running_expiration(121_001),
            Some(super::RuntimeExpiration {
                session_id: "runtime-operation-cancel".to_string(),
                reason: LeaseExpiry::Inactivity,
            })
        );
    }

    #[test]
    fn a_completed_owner_operation_renews_the_inactivity_deadline() {
        let slot = RuntimeSlot::new();
        let owner = project("operation-finish");
        commit_runtime(
            &slot,
            &owner,
            "runtime-operation-finish",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );
        slot.begin_owner_operation_at(&owner, "runtime-operation-finish", 100_000, 60_000)
            .unwrap()
            .finish(150_000);

        assert_eq!(slot.running_expiration(269_999), None);
        assert_eq!(
            slot.running_expiration(270_000).unwrap().reason,
            LeaseExpiry::Inactivity
        );
    }

    #[test]
    fn an_operation_that_outlives_its_bound_cannot_resurrect_the_lease() {
        let mut lease = RuntimeLease::new(1_000, None).unwrap();
        let monotonic_start = tokio::time::Instant::now();
        let (operation, _) = lease.begin_operation(monotonic_start, 10_000);

        lease.finish_operation_at(
            operation,
            110_000,
            monotonic_start + std::time::Duration::from_millis(10_000),
        );

        assert_eq!(
            lease.expiry_at(
                121_000,
                monotonic_start + std::time::Duration::from_millis(10_001)
            ),
            Some(LeaseExpiry::Inactivity)
        );
    }

    #[test]
    fn operation_suspension_is_capped_even_if_a_caller_requests_more() {
        let mut lease = RuntimeLease::new(1_000, None).unwrap();
        let monotonic_start = tokio::time::Instant::now();
        lease.begin_operation(monotonic_start, u64::MAX);

        assert_eq!(
            lease.expiry_at(
                219_999,
                monotonic_start + std::time::Duration::from_millis(119_999)
            ),
            None
        );
        assert_eq!(
            lease.expiry_at(
                220_000,
                monotonic_start + std::time::Duration::from_millis(120_000)
            ),
            Some(LeaseExpiry::Inactivity)
        );
    }

    #[test]
    fn forward_wall_clock_jump_does_not_end_a_monotonically_active_operation() {
        let mut lease = RuntimeLease::new(10_000, Some(DEFAULT_MAX_RUN_SECONDS)).unwrap();
        let monotonic_start = tokio::time::Instant::now();
        let (operation, _) = lease.begin_operation(monotonic_start, 60_000);

        assert_eq!(lease.expiry_at(500_000, monotonic_start), None);

        lease.cancel_operation(operation);
        assert_eq!(
            lease.expiry_at(500_000, monotonic_start),
            Some(LeaseExpiry::Inactivity)
        );
    }

    #[test]
    fn lease_limits_accept_long_regressions_and_reject_unsafe_values() {
        assert!(RuntimeLease::new(0, Some(4_500)).is_ok());
        assert!(RuntimeLease::new(0, Some(0)).is_err());
        assert!(RuntimeLease::new(0, Some(MAX_RUN_SECONDS + 1)).is_err());
    }

    #[test]
    fn lease_snapshots_expose_the_default_maximum_and_heartbeat_contract() {
        let default = RuntimeLease::new(10_000, None).unwrap().snapshot(10_000);
        assert_eq!(
            default.absolute_deadline_ms,
            10_000 + DEFAULT_MAX_RUN_SECONDS * 1_000
        );
        assert_eq!(default.absolute_remaining_seconds, 900);
        assert_eq!(default.inactivity_deadline_ms, Some(130_000));
        assert_eq!(default.inactivity_remaining_seconds, Some(120));
        assert_eq!(default.heartbeat_interval_ms, HEARTBEAT_INTERVAL_MS);

        let maximum = RuntimeLease::new(10_000, Some(MAX_RUN_SECONDS))
            .unwrap()
            .snapshot(10_000);
        assert_eq!(maximum.absolute_remaining_seconds, 86_400);
    }

    #[test]
    fn only_owner_activity_can_renew_a_running_lease() {
        let slot = RuntimeSlot::new();
        let owner = project("renew-owner");
        let outsider = project("renew-outsider");
        commit_runtime(
            &slot,
            &owner,
            "runtime-renew",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );

        assert!(matches!(
            slot.renew_and_observe(&outsider, None, 100_000),
            SlotObservation::Busy
        ));
        assert_eq!(
            slot.running_expiration(121_001).unwrap().reason,
            LeaseExpiry::Inactivity
        );
    }

    #[test]
    fn a_late_owner_heartbeat_cannot_resurrect_an_expired_lease() {
        let slot = RuntimeSlot::new();
        let owner = project("late-heartbeat");
        commit_runtime(
            &slot,
            &owner,
            "runtime-late-heartbeat",
            1_000,
            1_001,
            DEFAULT_MAX_RUN_SECONDS,
        );

        assert!(matches!(
            slot.renew_and_observe(&owner, None, 121_001),
            SlotObservation::Busy
        ));
        assert_eq!(
            slot.claim_expired(121_001).unwrap().reason(),
            Some(LeaseExpiry::Inactivity)
        );
    }

    #[test]
    fn claiming_an_expired_run_keeps_the_slot_busy_until_cleanup_finishes() {
        let slot = RuntimeSlot::new();
        let owner = project("expiry-claim-owner");
        let outsider = project("expiry-claim-outsider");
        commit_runtime(&slot, &owner, "expired-runtime", 1_000, 1_001, 1);

        assert!(slot.claim_expired(2_000).is_none());
        let expired = slot.claim_expired(2_001).unwrap();
        assert_eq!(expired.session_id(), "expired-runtime");
        assert_eq!(expired.reason(), Some(LeaseExpiry::Absolute));
        assert!(slot.is_occupied_now());
        assert!(matches!(
            slot.observe(&outsider, None, 2_001),
            SlotObservation::Busy
        ));
        assert!(matches!(
            slot.renew_and_observe(&owner, Some("expired-runtime"), 2_001),
            SlotObservation::Busy
        ));
        assert!(matches!(
            slot.renew_and_observe(&outsider, Some("expired-runtime"), 2_001),
            SlotObservation::NotOwnedOrFound
        ));

        expired.release_after_reap();
        assert!(!slot.is_occupied_now());
    }

    #[test]
    fn dropping_an_expiry_cleanup_claim_keeps_the_slot_fail_closed() {
        let slot = RuntimeSlot::new();
        let owner = project("expiry-drop");
        commit_runtime(&slot, &owner, "expired-runtime-drop", 1_000, 1_001, 1);

        drop(slot.claim_expired(2_001).unwrap());
        assert!(slot.is_occupied_now());
    }
}
