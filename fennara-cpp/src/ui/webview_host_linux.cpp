#ifdef __linux__

#include "webview_backend.hpp"

namespace fennara {
namespace webview_backend {

class LinuxWebviewBackend : public NativeWebviewBackend {
public:
    bool start(godot::Control *owner, const godot::String &url) override {
        (void)owner;
        (void)url;
        // Linux needs an in-process renderer; Wayland cannot reliably position a helper top-level window.
        output_error(
            "Web chat native dock webview is not implemented on Linux yet; "
            "future Linux support should use CEF off-screen rendering into an internal Godot texture.");
        return false;
    }

    void resize_to(godot::Control *owner) override {
        (void)owner;
    }

    void set_visible(bool visible) override {
        (void)visible;
    }

    void stop() override {
    }

    bool is_started() const override {
        return false;
    }
};

std::unique_ptr<NativeWebviewBackend> create_backend() {
    return std::make_unique<LinuxWebviewBackend>();
}

} // namespace webview_backend
} // namespace fennara

#endif
