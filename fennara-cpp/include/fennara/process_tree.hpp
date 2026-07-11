#pragma once

namespace fennara::process_tree {

// Terminates pid and all descendants, then waits for every captured process
// to exit before returning. Uses taskkill /T on Windows and recursive process
// discovery with TERM/KILL escalation on Unix platforms.
void terminate_and_wait(int pid, int timeout_ms = 5000);

} // namespace fennara::process_tree
