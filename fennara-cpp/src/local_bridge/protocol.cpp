#include "fennara/local_bridge.hpp"

#include "fennara/lsp/csharp_support.hpp"
#include "fennara/logger.hpp"
#include "fennara/tools/get_class_info/docs_branch.hpp"

#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/classes/crypto.hpp>
#include <godot_cpp/classes/marshalls.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/rendering_device.hpp>
#include <godot_cpp/classes/rendering_server.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/variant/array.hpp>

namespace fennara {

namespace {

constexpr int32_t MAX_CHAT_CONTEXT_SNIPPET_CHARS = 64000;

godot::String setting_value_or_empty(godot::ProjectSettings *settings,
                                     const godot::String &key) {
    if (settings == nullptr || !settings->has_setting(key)) {
        return "";
    }
    return godot::String(settings->get_setting(key, ""));
}

bool has_any_method(const godot::String &method, const godot::String &expected) {
    return method.strip_edges().to_lower() == expected;
}

godot::String device_type_name(godot::RenderingDevice::DeviceType type) {
    switch (type) {
        case godot::RenderingDevice::DEVICE_TYPE_INTEGRATED_GPU:
            return "integrated_gpu";
        case godot::RenderingDevice::DEVICE_TYPE_DISCRETE_GPU:
            return "discrete_gpu";
        case godot::RenderingDevice::DEVICE_TYPE_VIRTUAL_GPU:
            return "virtual_gpu";
        case godot::RenderingDevice::DEVICE_TYPE_CPU:
            return "cpu";
        case godot::RenderingDevice::DEVICE_TYPE_OTHER:
            return "other";
        default:
            return "unknown";
    }
}

godot::Array active_os_feature_tags(godot::OS *os) {
    godot::Array features;
    if (os == nullptr) {
        return features;
    }

    static const char *candidates[] = {
        "editor",
        "template",
        "debug",
        "release",
        "windows",
        "macos",
        "linux",
        "bsd",
        "android",
        "ios",
        "web",
        "mobile",
        "pc",
        "x11",
        "wayland",
        "server",
        "headless",
    };

    for (const char *candidate : candidates) {
        godot::String feature(candidate);
        if (os->has_feature(feature)) {
            features.append(feature);
        }
    }
    return features;
}

} // namespace

void FennaraLocalBridge::_send_hello() {
    godot::Dictionary payload;
    payload["type"] = "hello";
    payload["session_id"] = _session_id;
    payload["project_name"] = _project_name();
    payload["project_path"] = _project_path();
    payload["godot_executable_path"] = _godot_executable_path();
    payload["plugin_version"] = PLUGIN_VERSION;
    payload["chat_token"] = _chat_token;
    payload["godot_version"] = godot::String(godot::Engine::get_singleton()->get_version_info()["string"]);
    payload["csharp_support"] = csharp_support::inspect_project();
    payload["rendering_context"] = collect_rendering_context();

    godot::Array tools;
    tools.append("read_file");
    tools.append("write_or_update_file");
    tools.append("run_scene_edit_script");
    tools.append("get_scene_tree");
    tools.append("script_diagnostics");
    tools.append("screenshot_scene");
    tools.append("get_node_properties");
    tools.append("get_class_info");
    tools.append("validate_scene");
    tools.append("project_settings");
    tools.append("runtime_session");
    tools.append("runtime_script");
    tools.append("scrape_editor");
    payload["tools"] = tools;

    godot::String body = godot::JSON::stringify(payload);
    godot::Error err = _ws->send_text(body);
    if (err == godot::OK) {
        _sent_hello = true;
        FLOG_NET("Local bridge hello sent");
    } else {
        FLOG_ERR("Local bridge failed to send hello");
    }
}

godot::Dictionary FennaraLocalBridge::collect_rendering_context() {
    godot::Dictionary context;
    context["schema_version"] = "rendering-context-v1";

    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    godot::String project_method =
        setting_value_or_empty(settings, "rendering/renderer/rendering_method");
    godot::String project_method_mobile =
        setting_value_or_empty(settings, "rendering/renderer/rendering_method.mobile");
    godot::String project_method_web =
        setting_value_or_empty(settings, "rendering/renderer/rendering_method.web");
    godot::String project_device_driver =
        setting_value_or_empty(settings, "rendering/rendering_device/driver");

    godot::Dictionary project_settings;
    project_settings["rendering/renderer/rendering_method"] = project_method;
    project_settings["rendering/renderer/rendering_method.mobile"] = project_method_mobile;
    project_settings["rendering/renderer/rendering_method.web"] = project_method_web;
    project_settings["rendering/rendering_device/driver"] = project_device_driver;
    context["project_settings"] = project_settings;
    context["project_rendering_method"] = project_method;
    context["project_rendering_method_mobile"] = project_method_mobile;
    context["project_rendering_method_web"] = project_method_web;
    context["project_rendering_device_driver"] = project_device_driver;

    godot::RenderingServer *server = godot::RenderingServer::get_singleton();
    godot::String runtime_method;
    godot::String runtime_driver;
    bool has_rendering_device = false;
    if (server != nullptr) {
        runtime_method = server->get_current_rendering_method();
        runtime_driver = server->get_current_rendering_driver_name();
        has_rendering_device = server->get_rendering_device() != nullptr;
        context["video_adapter_name"] = server->get_video_adapter_name();
        context["video_adapter_vendor"] = server->get_video_adapter_vendor();
        context["video_adapter_type"] = device_type_name(server->get_video_adapter_type());
        context["video_adapter_api_version"] = server->get_video_adapter_api_version();
    } else {
        context["video_adapter_name"] = "";
        context["video_adapter_vendor"] = "";
        context["video_adapter_type"] = "";
        context["video_adapter_api_version"] = "";
    }
    context["runtime_rendering_method"] = runtime_method;
    context["runtime_rendering_driver_name"] = runtime_driver;
    context["has_rendering_device"] = has_rendering_device;

    godot::OS *os = godot::OS::get_singleton();
    context["os_name"] = os != nullptr ? os->get_name() : godot::String();
    context["os_distribution_name"] = os != nullptr ? os->get_distribution_name() : godot::String();
    context["os_version"] = os != nullptr ? os->get_version() : godot::String();
    context["os_model_name"] = os != nullptr ? os->get_model_name() : godot::String();
    context["os_feature_tags"] = active_os_feature_tags(os);

    bool runtime_compat = has_any_method(runtime_method, "gl_compatibility");
    bool project_compat = has_any_method(project_method, "gl_compatibility") ||
                          has_any_method(project_method_mobile, "gl_compatibility") ||
                          has_any_method(project_method_web, "gl_compatibility");
    bool runtime_mobile = has_any_method(runtime_method, "mobile");
    bool project_mobile = has_any_method(project_method, "mobile") ||
                          has_any_method(project_method_mobile, "mobile") ||
                          has_any_method(project_method_web, "mobile");
    bool runtime_forward_plus = has_any_method(runtime_method, "forward_plus");
    bool project_forward_plus = has_any_method(project_method, "forward_plus");

    context["is_compatibility"] = runtime_compat || project_compat;
    context["is_mobile_renderer"] = runtime_mobile || project_mobile;
    context["is_forward_plus"] = runtime_forward_plus || project_forward_plus;
    bool renderer_setting_mismatch =
        !project_method.is_empty() && !runtime_method.is_empty() &&
        project_method.strip_edges().to_lower() != runtime_method.strip_edges().to_lower();
    context["renderer_setting_mismatch"] = renderer_setting_mismatch;

    godot::Array warnings;
    if (runtime_compat || project_compat) {
        warnings.append("Compatibility/OpenGL renderer is active or configured; verify shader, screen/depth texture, compute, post-processing, lighting, particles, and advanced 3D feature support before suggesting changes.");
    }
    if (runtime_mobile || project_mobile) {
        warnings.append("Mobile renderer is active or configured; check advanced 3D effects, post-processing, particles, light counts, and texture/render target assumptions.");
    }
    if (renderer_setting_mismatch) {
        warnings.append("Project rendering method differs from the current runtime rendering method; prefer runtime values for the connected editor session and inspect project overrides before changing renderer-sensitive assets.");
    }
    if (!has_rendering_device) {
        warnings.append("RenderingDevice is unavailable from the current renderer; compute shader and low-level RenderingDevice suggestions are unsafe unless another target explicitly supports them.");
    }
    context["warnings"] = warnings;

    return context;
}

void FennaraLocalBridge::request_get_class_info_warmup() {
    _queued_get_class_info_warmup = true;
    _maybe_send_get_class_info_warmup();
}

void FennaraLocalBridge::_maybe_send_get_class_info_warmup() {
    if (!_queued_get_class_info_warmup || _sent_get_class_info_warmup ||
        !_sent_hello || !_ws.is_valid() ||
        _ws->get_ready_state() != godot::WebSocketPeer::STATE_OPEN) {
        return;
    }

    godot::Dictionary payload;
    payload["type"] = "warm_get_class_info_docs";
    payload["branch"] = get_class_info::docs_branch_for_running_godot();

    godot::Array class_names;
    class_names.append("Object");
    class_names.append("RefCounted");
    class_names.append("Resource");
    class_names.append("Node");
    class_names.append("Node2D");
    class_names.append("Node3D");
    class_names.append("CanvasItem");
    class_names.append("Control");
    class_names.append("Sprite2D");
    class_names.append("AnimatedSprite2D");
    class_names.append("CharacterBody2D");
    class_names.append("RigidBody2D");
    class_names.append("StaticBody2D");
    class_names.append("Area2D");
    class_names.append("Camera2D");
    class_names.append("TileMap");
    class_names.append("CollisionShape2D");
    class_names.append("CollisionPolygon2D");
    class_names.append("RayCast2D");
    class_names.append("Marker2D");
    class_names.append("Path2D");
    class_names.append("PathFollow2D");
    class_names.append("Texture2D");
    class_names.append("Image");
    class_names.append("AtlasTexture");
    class_names.append("AnimationPlayer");
    class_names.append("GPUParticles2D");
    class_names.append("AudioStreamPlayer2D");
    class_names.append("Animation");
    class_names.append("AnimationLibrary");
    class_names.append("Curve");
    class_names.append("Curve2D");
    class_names.append("Shape2D");
    class_names.append("RectangleShape2D");
    class_names.append("CircleShape2D");
    class_names.append("CapsuleShape2D");
    class_names.append("World2D");
    class_names.append("ParticleProcessMaterial");
    class_names.append("Shader");
    class_names.append("Material");
    class_names.append("CanvasItemMaterial");
    class_names.append("AudioStream");
    class_names.append("AudioStreamWAV");
    class_names.append("Label");
    class_names.append("Button");
    class_names.append("TextureButton");
    class_names.append("LineEdit");
    class_names.append("TextEdit");
    class_names.append("RichTextLabel");
    class_names.append("Panel");
    class_names.append("PanelContainer");
    class_names.append("MarginContainer");
    class_names.append("VBoxContainer");
    class_names.append("HBoxContainer");
    class_names.append("GridContainer");
    class_names.append("ScrollContainer");
    class_names.append("TabContainer");
    class_names.append("ColorRect");
    class_names.append("TextureRect");
    class_names.append("OptionButton");
    class_names.append("CheckBox");
    class_names.append("Slider");
    class_names.append("ProgressBar");
    class_names.append("ItemList");
    class_names.append("Tree");
    class_names.append("TabBar");
    class_names.append("Window");
    class_names.append("Theme");
    class_names.append("StyleBox");
    class_names.append("StyleBoxFlat");
    class_names.append("StyleBoxTexture");
    class_names.append("Font");
    class_names.append("FontFile");
    class_names.append("FontVariation");
    class_names.append("ShaderMaterial");
    class_names.append("Gradient");
    class_names.append("GradientTexture1D");
    class_names.append("GradientTexture2D");
    class_names.append("MeshInstance3D");
    class_names.append("Sprite3D");
    class_names.append("CharacterBody3D");
    class_names.append("RigidBody3D");
    class_names.append("StaticBody3D");
    class_names.append("Area3D");
    class_names.append("Camera3D");
    class_names.append("Marker3D");
    class_names.append("RayCast3D");
    class_names.append("CollisionShape3D");
    class_names.append("CollisionPolygon3D");
    class_names.append("CSGBox3D");
    class_names.append("GPUParticles3D");
    class_names.append("DirectionalLight3D");
    class_names.append("OmniLight3D");
    class_names.append("SpotLight3D");
    class_names.append("WorldEnvironment");
    class_names.append("NavigationRegion3D");
    class_names.append("AudioStreamPlayer3D");
    class_names.append("Mesh");
    class_names.append("ArrayMesh");
    class_names.append("PrimitiveMesh");
    class_names.append("BoxMesh");
    class_names.append("SphereMesh");
    class_names.append("CylinderMesh");
    class_names.append("PlaneMesh");
    class_names.append("BaseMaterial3D");
    class_names.append("StandardMaterial3D");
    class_names.append("ORMMaterial3D");
    class_names.append("Environment");
    class_names.append("World3D");
    class_names.append("Shape3D");
    class_names.append("BoxShape3D");
    class_names.append("SphereShape3D");
    class_names.append("CapsuleShape3D");
    class_names.append("ConcavePolygonShape3D");
    class_names.append("ConvexPolygonShape3D");
    class_names.append("SpriteFrames");
    payload["class_names"] = class_names;

    _send_json(payload);
    _sent_get_class_info_warmup = true;
}

bool FennaraLocalBridge::set_as_active_project() {
    if (!_ws.is_valid() || _ws->get_ready_state() != godot::WebSocketPeer::STATE_OPEN) {
        return false;
    }

    godot::Dictionary payload;
    payload["type"] = "set_active_project";
    payload["session_id"] = _session_id;
    _send_json(payload);
    _active_mcp_target_name = _project_name();
    _active_mcp_target_path = _project_path();
    if (!_is_active_mcp_target) {
        _is_active_mcp_target = true;
        emit_signal("mcp_target_state_changed", true);
    }
    FLOG_NET("Local bridge requested active MCP project");
    return true;
}

bool FennaraLocalBridge::send_chat_context_snippet(const godot::String &path,
                                                   int32_t start_line,
                                                   int32_t end_line,
                                                   const godot::String &text) {
    if (!is_daemon_connected()) {
        return false;
    }

    godot::String clean_path = path.strip_edges();
    godot::String clean_text = text.replace("\r\n", "\n").replace("\r", "\n");
    if (clean_path.is_empty() || clean_text.strip_edges().is_empty() ||
        start_line <= 0 || end_line < start_line) {
        return false;
    }

    if (clean_text.length() > MAX_CHAT_CONTEXT_SNIPPET_CHARS) {
        clean_text = clean_text.substr(0, MAX_CHAT_CONTEXT_SNIPPET_CHARS) +
                     "\n... [truncated by Fennara]\n";
    }

    godot::Dictionary payload;
    payload["type"] = "chat_context_snippet";
    payload["session_id"] = _session_id;
    payload["path"] = clean_path;
    payload["start_line"] = start_line;
    payload["end_line"] = end_line;
    payload["text"] = clean_text;
    _send_json(payload);
    return true;
}

void FennaraLocalBridge::_send_json(const godot::Dictionary &payload) {
    if (!_ws.is_valid() || _ws->get_ready_state() != godot::WebSocketPeer::STATE_OPEN) {
        return;
    }

    godot::String body = godot::JSON::stringify(payload);
    godot::Error err = _ws->send_text(body);
    if (err != godot::OK) {
        FLOG_ERR("Local bridge failed to send JSON payload");
    }
}

godot::String FennaraLocalBridge::_make_session_id() const {
    return _project_path() + "#" + godot::String::num_int64(godot::OS::get_singleton()->get_process_id());
}

godot::String FennaraLocalBridge::_make_chat_token() const {
    godot::Ref<godot::Crypto> crypto;
    crypto.instantiate();
    if (crypto.is_valid()) {
        godot::PackedByteArray bytes = crypto->generate_random_bytes(32);
        return godot::Marshalls::get_singleton()->raw_to_base64(bytes)
            .replace("+", "-")
            .replace("/", "_")
            .replace("=", "");
    }
    return _session_id.md5_text() + godot::String::num_int64(godot::Time::get_singleton()->get_ticks_msec());
}

godot::String FennaraLocalBridge::_project_name() const {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        return "";
    }

    return settings->get_setting("application/config/name", "");
}

godot::String FennaraLocalBridge::_project_path() const {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        return "";
    }

    return settings->globalize_path("res://");
}

godot::String FennaraLocalBridge::_godot_executable_path() const {
    godot::OS *os = godot::OS::get_singleton();
    if (os == nullptr) {
        return "";
    }

    return os->get_executable_path().strip_edges();
}

} // namespace fennara
