#ifdef __linux__

#include "linux_cef_bridge_loader.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/project_settings.hpp>

#include <dlfcn.h>

#include <string>

namespace fennara::linux_cef_bridge_loader {
namespace {

constexpr const char *kBridgeAddonPath = "res://addons/fennara/bin/libfennara_linux_cef_bridge.so";

std::string utf8(const godot::String &value) {
    return value.utf8().get_data();
}

godot::String bridge_library_path() {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        return kBridgeAddonPath;
    }
    return settings->globalize_path(kBridgeAddonPath);
}

} // namespace

BridgeLibrary::~BridgeLibrary() {
    close();
}

bool BridgeLibrary::load(godot::String &error_message) {
    library_path = bridge_library_path();
    if (!godot::FileAccess::file_exists(library_path)) {
        error_message = "Linux CEF bridge library is missing: " + library_path;
        return false;
    }

    dlerror();
    handle = dlopen(utf8(library_path).c_str(), RTLD_NOW | RTLD_LOCAL);
    if (handle == nullptr) {
        const char *error = dlerror();
        error_message = godot::String("Linux CEF bridge library could not be opened: ") +
                        library_path +
                        (error != nullptr ? godot::String(" (") + error + ")" : godot::String());
        return false;
    }

    dlerror();
    auto get_api = reinterpret_cast<fennara_linux_cef_bridge_get_api_t>(
        dlsym(handle, "fennara_linux_cef_bridge_get_api"));
    const char *symbol_error = dlerror();
    if (symbol_error != nullptr || get_api == nullptr) {
        error_message = "Linux CEF bridge library is missing fennara_linux_cef_bridge_get_api: " + library_path;
        close();
        return false;
    }

    api_ptr = get_api(FENNARA_LINUX_CEF_BRIDGE_API_VERSION);
    if (api_ptr == nullptr ||
        api_ptr->version != FENNARA_LINUX_CEF_BRIDGE_API_VERSION ||
        api_ptr->size < sizeof(fennara_cef_bridge_api)) {
        error_message = "Linux CEF bridge library has an incompatible API version: " + library_path;
        close();
        return false;
    }

    return true;
}

void BridgeLibrary::close() {
    api_ptr = nullptr;
    if (handle != nullptr) {
        dlclose(handle);
        handle = nullptr;
    }
}

const fennara_cef_bridge_api *BridgeLibrary::api() const {
    return api_ptr;
}

godot::String BridgeLibrary::path() const {
    return library_path;
}

} // namespace fennara::linux_cef_bridge_loader

#endif
