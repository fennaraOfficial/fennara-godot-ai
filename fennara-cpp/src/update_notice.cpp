#include "fennara/update_notice.hpp"

#include "fennara/logger.hpp"
#include "fennara/release/discovery.hpp"

#include <godot_cpp/variant/variant.hpp>

namespace fennara::update_notice {
namespace {

bool g_checked = false;
release_discovery::Result g_result;

} // namespace

void check_once() {
    if (g_checked) {
        return;
    }
    g_checked = true;
    g_result = release_discovery::check(5000);
    if (!g_result.success) {
        FLOG_TOOL("Update check skipped: " + g_result.error);
    }
}

bool is_update_available() {
    return g_result.update_available;
}

godot::String current_version() {
    return g_result.current.version;
}

godot::String latest_version() {
    return g_result.target_version;
}

godot::String channel() {
    return g_result.current.channel;
}

godot::String track() {
    return g_result.current.track;
}

godot::String target_release_tag() {
    return g_result.target_release_tag;
}

godot::String source_commit() {
    return g_result.target_source_commit.is_empty() ? g_result.current.source_commit
                                                    : g_result.target_source_commit;
}

godot::String warning_text() {
    if (!g_result.update_available) {
        return "";
    }
    const godot::String label = g_result.current.is_staging()
                                    ? g_result.current.channel + " staging"
                                    : godot::String("stable");
    return "Fennara " + label + " is out of date. Current addon: " +
           current_version() + ". Available release: " + latest_version() + ".";
}

godot::Dictionary status() {
    godot::Dictionary result;
    result["checked"] = g_checked;
    result["check_failed"] = g_checked && !g_result.success;
    result["track"] = track();
    result["channel"] = channel();
    result["current_version"] = current_version();
    result["latest_version"] = latest_version();
    result["target_release_tag"] = target_release_tag();
    result["source_commit"] = source_commit();
    result["installed_source_commit"] = g_result.current.source_commit;
    result["outdated"] = g_result.update_available;
    result["message"] = g_result.update_available ? warning_text() : g_result.detail;
    if (!g_result.error.is_empty()) {
        result["error"] = g_result.error;
    }
    return result;
}

} // namespace fennara::update_notice
