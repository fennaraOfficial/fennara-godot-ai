#pragma once

#ifdef __linux__

#include "linux_cef_bridge_api.hpp"

#include <godot_cpp/variant/string.hpp>

namespace fennara::linux_cef_bridge_loader {

class BridgeLibrary {
public:
    BridgeLibrary() = default;
    BridgeLibrary(const BridgeLibrary &) = delete;
    BridgeLibrary &operator=(const BridgeLibrary &) = delete;
    ~BridgeLibrary();

    bool load(godot::String &error_message);
    void close();

    const fennara_cef_bridge_api *api() const;
    godot::String path() const;

private:
    void *handle = nullptr;
    const fennara_cef_bridge_api *api_ptr = nullptr;
    godot::String library_path;
};

} // namespace fennara::linux_cef_bridge_loader

#endif
