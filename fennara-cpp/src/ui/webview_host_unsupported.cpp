#if !defined(_WIN32) && !defined(__APPLE__) && !defined(__linux__)

#include "webview_backend.hpp"

namespace fennara {
namespace webview_backend {

class UnsupportedWebviewBackend : public NativeWebviewBackend {
public:
    bool start(godot::Control *owner, const godot::String &url) override {
        (void)owner;
        (void)url;
        output_error("Web chat native webview is not wired for this platform build yet");
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
    return std::make_unique<UnsupportedWebviewBackend>();
}

} // namespace webview_backend
} // namespace fennara

#endif
