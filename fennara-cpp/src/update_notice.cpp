#include "fennara/update_notice.hpp"

#include "fennara/logger.hpp"
#include "fennara/release/discovery.hpp"

#include <godot_cpp/variant/variant.hpp>

#include <mutex>

namespace fennara::update_notice {
namespace {

bool g_check_started = false;
bool g_checked = false;
release_discovery::Result g_result;
std::mutex g_mutex;

struct Snapshot {
    bool check_started = false;
    bool checked = false;
    release_discovery::Result result;
};

Snapshot snapshot() {
    std::lock_guard<std::mutex> lock(g_mutex);
    return {g_check_started, g_checked, g_result};
}

godot::String warning_text_for(const release_discovery::Result &result) {
    if (!result.update_available) {
        return "";
    }
    const godot::String label = result.current.is_staging()
                                    ? result.current.channel + godot::String(" staging")
                                    : godot::String("stable");
    return "Fennara " + label + " is out of date. Current addon: " +
           result.current.version + ". Available release: " + result.target_version + ".";
}

} // namespace

void check_once(const std::atomic_bool *cancelled) {
    {
        std::lock_guard<std::mutex> lock(g_mutex);
        if (g_check_started) {
            return;
        }
        g_check_started = true;
    }
    release_discovery::Result result = release_discovery::check(5000, cancelled);
    if (!result.success) {
        FLOG_TOOL("Update check skipped: " + result.error);
    }
    {
        std::lock_guard<std::mutex> lock(g_mutex);
        if (result.cancelled) {
            g_check_started = false;
            g_checked = false;
            return;
        }
        g_result = result;
        g_checked = true;
    }
}

bool is_update_available() {
    return snapshot().result.update_available;
}

godot::String current_version() {
    return snapshot().result.current.version;
}

godot::String latest_version() {
    return snapshot().result.target_version;
}

godot::String channel() {
    return snapshot().result.current.channel;
}

godot::String track() {
    return snapshot().result.current.track;
}

godot::String target_release_tag() {
    return snapshot().result.target_release_tag;
}

godot::String source_commit() {
    const release_discovery::Result result = snapshot().result;
    return result.target_source_commit.is_empty() ? result.current.source_commit
                                                  : result.target_source_commit;
}

godot::String warning_text() {
    return warning_text_for(snapshot().result);
}

godot::Dictionary status() {
    const Snapshot state = snapshot();
    const release_discovery::Result &result_state = state.result;
    godot::Dictionary result;
    result["checking"] = state.check_started && !state.checked;
    result["checked"] = state.checked;
    result["check_failed"] = state.checked && !result_state.success;
    result["track"] = result_state.current.track;
    result["channel"] = result_state.current.channel;
    result["current_version"] = result_state.current.version;
    result["latest_version"] = result_state.target_version;
    result["target_release_tag"] = result_state.target_release_tag;
    result["source_commit"] = result_state.target_source_commit.is_empty()
                                  ? result_state.current.source_commit
                                  : result_state.target_source_commit;
    result["installed_source_commit"] = result_state.current.source_commit;
    result["outdated"] = result_state.update_available;
    result["message"] = result_state.update_available ? warning_text_for(result_state)
                                                       : result_state.detail;
    if (!result_state.error.is_empty()) {
        result["error"] = result_state.error;
    }
    return result;
}

} // namespace fennara::update_notice
