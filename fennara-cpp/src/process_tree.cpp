#include "fennara/process_tree.hpp"

#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

#include <algorithm>
#include <chrono>
#include <thread>
#include <unordered_set>
#include <utility>
#include <vector>

namespace fennara::process_tree {
namespace {

std::vector<int> unix_descendants(int root_pid) {
    godot::PackedStringArray args;
    args.append("-eo");
    args.append("pid=,ppid=");
    godot::Array output;
    int exit_code = godot::OS::get_singleton()->execute(
        "ps", args, output, true, false);
    if (exit_code != 0 || output.is_empty()) {
        return {};
    }

    std::vector<std::pair<int, int>> relationships;
    godot::PackedStringArray lines =
        godot::String(output[0]).split("\n", false);
    for (int i = 0; i < lines.size(); i++) {
        godot::PackedStringArray fields =
            lines[i].strip_edges().split(" ", false);
        if (fields.size() < 2) {
            continue;
        }
        relationships.emplace_back(
            static_cast<int>(fields[0].to_int()),
            static_cast<int>(fields[1].to_int()));
    }

    std::unordered_set<int> family{root_pid};
    std::vector<int> descendants;
    bool added = true;
    while (added) {
        added = false;
        for (const auto &[pid, parent] : relationships) {
            if (family.find(parent) == family.end() ||
                family.find(pid) != family.end()) {
                continue;
            }
            family.insert(pid);
            descendants.push_back(pid);
            added = true;
        }
    }
    std::reverse(descendants.begin(), descendants.end());
    return descendants;
}

bool any_running(const std::vector<int> &pids) {
    godot::OS *os = godot::OS::get_singleton();
    for (int pid : pids) {
        if (pid > 0 && os->is_process_running(pid)) {
            return true;
        }
    }
    return false;
}

void wait_until_stopped(const std::vector<int> &pids, uint64_t deadline_ms) {
    while (any_running(pids) &&
           godot::Time::get_singleton()->get_ticks_msec() < deadline_ms) {
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
}

} // namespace

void terminate_and_wait(int pid, int timeout_ms) {
    if (pid <= 0) {
        return;
    }
    godot::OS *os = godot::OS::get_singleton();
    if (os == nullptr) {
        return;
    }

    uint64_t started = godot::Time::get_singleton()->get_ticks_msec();
    uint64_t deadline = started + static_cast<uint64_t>(timeout_ms);
    godot::Array output;
    if (os->get_name() == "Windows") {
        godot::PackedStringArray args;
        args.append("/PID");
        args.append(godot::String::num_int64(pid));
        args.append("/T");
        args.append("/F");
        os->execute("taskkill", args, output, true, false);
        wait_until_stopped({pid}, deadline);
        if (os->is_process_running(pid)) {
            os->kill(pid);
        }
        return;
    }

    std::vector<int> family = unix_descendants(pid);
    family.push_back(pid);
    godot::PackedStringArray term_args;
    term_args.append("-TERM");
    for (int family_pid : family) {
        term_args.append(godot::String::num_int64(family_pid));
    }
    os->execute("kill", term_args, output, true, false);

    uint64_t term_deadline = std::min(
        deadline, started + static_cast<uint64_t>(timeout_ms / 2));
    wait_until_stopped(family, term_deadline);
    for (int family_pid : family) {
        if (os->is_process_running(family_pid)) {
            os->kill(family_pid);
        }
    }
    wait_until_stopped(family, deadline);
}

} // namespace fennara::process_tree
