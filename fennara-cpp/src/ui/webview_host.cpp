#include "fennara/ui/webview_host.hpp"

#include "webview_backend.hpp"

#include "fennara/logger.hpp"

#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/variant/utility_functions.hpp>

namespace fennara {

namespace {

bool editor_is_headless() {
    godot::DisplayServer *display = godot::DisplayServer::get_singleton();
    godot::OS *os = godot::OS::get_singleton();
    return (os != nullptr && os->has_feature("headless")) ||
           (display != nullptr && display->get_name().to_lower() == "headless");
}

} // namespace

namespace webview_backend {

void output_log(const godot::String &message) {
    FLOG_UI(message);
    godot::UtilityFunctions::print(godot::String("[Fennara] ") + message);
}

void output_error(const godot::String &message) {
    FLOG_ERR(message);
    godot::UtilityFunctions::push_error(godot::String("[Fennara] ") + message);
}

} // namespace webview_backend

WebviewHost::WebviewHost() :
        backend(webview_backend::create_backend()) {
}

WebviewHost::~WebviewHost() {
    stop();
}

bool WebviewHost::start(godot::Control *owner, const godot::String &url) {
    if (is_started()) {
        webview_backend::output_log("Web chat host already started");
        return true;
    }

    if (editor_is_headless()) {
        webview_backend::output_log("Web chat host skipped: headless editor has no native window");
        return false;
    }

    if (backend == nullptr) {
        webview_backend::output_error("Web chat native webview is not wired for this platform build yet");
        return false;
    }
    return backend->start(owner, url);
}

void WebviewHost::resize_to(godot::Control *owner) {
    if (backend != nullptr) {
        backend->resize_to(owner);
    }
}

void WebviewHost::set_visible(bool visible) {
    if (backend != nullptr) {
        backend->set_visible(visible);
    }
}

void WebviewHost::stop() {
    if (backend != nullptr) {
        backend->stop();
    }
}

bool WebviewHost::is_started() const {
    return backend != nullptr && backend->is_started();
}

} // namespace fennara
