#[path = "../runtime_daemon/mod.rs"]
mod runtime_daemon;

#[tokio::main]
async fn main() {
    runtime_daemon::run().await;
}
