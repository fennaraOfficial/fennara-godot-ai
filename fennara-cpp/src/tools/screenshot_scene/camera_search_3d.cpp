#include "fennara/tools/screenshot_scene.hpp"

#include <algorithm>
#include <cmath>
#include <limits>
#include <memory>
#include <vector>

#include <godot_cpp/classes/camera3d.hpp>
#include <godot_cpp/classes/canvas_item.hpp>
#include <godot_cpp/classes/environment.hpp>
#include <godot_cpp/classes/geometry_instance3d.hpp>
#include <godot_cpp/classes/image.hpp>
#include <godot_cpp/classes/shader.hpp>
#include <godot_cpp/classes/shader_material.hpp>
#include <godot_cpp/classes/sub_viewport.hpp>
#include <godot_cpp/classes/viewport_texture.hpp>

namespace fennara {

namespace {

struct MaterialState {
    godot::GeometryInstance3D *geometry = nullptr;
    godot::Ref<godot::Material> material;
    int subject_index = -1;
};

struct CanvasVisibilityState {
    godot::CanvasItem *item = nullptr;
    bool visible = false;
};

struct Candidate {
    int azimuth = 0;
    int elevation = 0;
    godot::Vector3 direction;
};

constexpr int SUBJECT_COLOR_COUNT =
    SCREENSHOT_CAMERA_SEARCH_SUBJECT_LIMIT;

godot::Color subject_color(int index) {
    switch (index) {
        case 0: return godot::Color(0.216f, 0.835f, 1.000f, 1.0f);
        case 1: return godot::Color(1.000f, 0.624f, 0.263f, 1.0f);
        case 2: return godot::Color(0.659f, 1.000f, 0.243f, 1.0f);
        case 3: return godot::Color(1.000f, 0.310f, 0.702f, 1.0f);
        case 4: return godot::Color(1.000f, 0.882f, 0.290f, 1.0f);
        case 5: return godot::Color(0.616f, 0.439f, 1.000f, 1.0f);
        case 6: return godot::Color(1.000f, 0.365f, 0.365f, 1.0f);
        default: return godot::Color(0.212f, 0.878f, 0.627f, 1.0f);
    }
}

void collect_render_state(godot::Node *node, const godot::Array &subjects,
                          std::vector<MaterialState> &materials,
                          std::vector<CanvasVisibilityState> &canvas) {
    if (!node) return;

    if (auto *geometry =
            godot::Object::cast_to<godot::GeometryInstance3D>(node)) {
        MaterialState state;
        state.geometry = geometry;
        state.material = geometry->get_material_override();
        for (int i = 0; i < subjects.size(); i++) {
            godot::Object *object = subjects[i];
            godot::Node *subject = godot::Object::cast_to<godot::Node>(object);
            if (subject && (subject == node || subject->is_ancestor_of(node))) {
                state.subject_index = i;
                break;
            }
        }
        materials.push_back(state);
    }
    if (auto *item = godot::Object::cast_to<godot::CanvasItem>(node)) {
        canvas.push_back({item, item->is_visible()});
    }
    for (int i = 0; i < node->get_child_count(); i++) {
        collect_render_state(node->get_child(i), subjects, materials, canvas);
    }
}

godot::Ref<godot::ShaderMaterial> make_flat_material(
    const godot::Color &color) {
    static godot::Ref<godot::Shader> *shared_shader = nullptr;
    if (shared_shader == nullptr) {
        shared_shader = new godot::Ref<godot::Shader>;
        shared_shader->instantiate();
        (*shared_shader)->set_code(
            "shader_type spatial;\n"
            "render_mode unshaded, fog_disabled, shadows_disabled;\n"
            "uniform vec4 mask_color : source_color;\n"
            "void fragment() { ALBEDO = mask_color.rgb; ALPHA = 1.0; }\n");
    }
    godot::Ref<godot::ShaderMaterial> material;
    material.instantiate();
    material->set_shader(*shared_shader);
    material->set_shader_parameter("mask_color", color);
    return material;
}

void set_synthetic_background(
    const std::vector<CanvasVisibilityState> &canvas, bool enabled) {
    for (const CanvasVisibilityState &state : canvas) {
        if (state.item) state.item->set_visible(enabled ? false : state.visible);
    }
}

void restore_materials(const std::vector<MaterialState> &materials) {
    for (const MaterialState &state : materials) {
        if (state.geometry) state.geometry->set_material_override(state.material);
    }
}

struct MaskBounds {
    int min_x = 0;
    int min_y = 0;
    int max_x = -1;
    int max_y = -1;
    int pixels = 0;
};

int closest_subject_color(const godot::Color &pixel, int subject_count) {
    int best = -1;
    float best_distance = 100.0f;
    for (int i = 0; i < std::min(subject_count, SUBJECT_COLOR_COUNT); i++) {
        godot::Color expected = subject_color(i);
        float dr = pixel.r - expected.r;
        float dg = pixel.g - expected.g;
        float db = pixel.b - expected.b;
        float distance = dr * dr + dg * dg + db * db;
        if (distance < best_distance) {
            best_distance = distance;
            best = i;
        }
    }
    return best_distance <= 0.12f ? best : -1;
}

godot::Array measure_subjects(const godot::Ref<godot::Image> &mask,
                               godot::Node *root,
                               const godot::Array &subjects) {
    godot::Array receipt;
    if (mask.is_null()) return receipt;
    if (mask->get_format() != godot::Image::FORMAT_RGBA8) {
        mask->convert(godot::Image::FORMAT_RGBA8);
    }
    const int width = mask->get_width();
    const int height = mask->get_height();
    const int subject_count = std::min(int(subjects.size()), 8);
    std::vector<MaskBounds> bounds(subject_count);
    godot::PackedByteArray pixel_data = mask->get_data();
    const uint8_t *pixels = pixel_data.ptr();
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            const int offset = (y * width + x) * 4;
            godot::Color pixel(
                float(pixels[offset]) / 255.0f,
                float(pixels[offset + 1]) / 255.0f,
                float(pixels[offset + 2]) / 255.0f,
                float(pixels[offset + 3]) / 255.0f);
            int id = closest_subject_color(pixel, subject_count);
            if (id < 0) continue;
            MaskBounds &box = bounds[id];
            if (box.pixels == 0) {
                box.min_x = box.max_x = x;
                box.min_y = box.max_y = y;
            } else {
                box.min_x = std::min(box.min_x, x);
                box.min_y = std::min(box.min_y, y);
                box.max_x = std::max(box.max_x, x);
                box.max_y = std::max(box.max_y, y);
            }
            box.pixels++;
        }
    }

    for (int i = 0; i < subjects.size(); i++) {
        godot::Object *object = subjects[i];
        godot::Node *subject = godot::Object::cast_to<godot::Node>(object);
        godot::String path = subject && root
            ? godot::String(root->get_path_to(subject))
            : godot::String("unknown");
        godot::Dictionary item;
        item["path"] = path;
        bool visible = i < subject_count && bounds[i].pixels > 0;
        item["visible"] = visible;
        item["visible_pixels"] = visible ? bounds[i].pixels : 0;
        if (visible) {
            MaskBounds &box = bounds[i];
            godot::Dictionary screen_rect;
            screen_rect["x"] = box.min_x;
            screen_rect["y"] = box.min_y;
            screen_rect["width"] = box.max_x - box.min_x + 1;
            screen_rect["height"] = box.max_y - box.min_y + 1;
            item["screen_rect"] = screen_rect;

        }
        receipt.append(item);
    }
    return receipt;
}

double fit_perspective_distance(const godot::Vector3 &center,
                                const godot::Vector3 &size,
                                const godot::Vector3 &view_dir,
                                float aspect, float margin) {
    godot::Vector3 up(0, 1, 0);
    godot::Vector3 right = up.cross(view_dir).normalized();
    godot::Vector3 camera_up = view_dir.cross(right).normalized();
    double fov_rad = 70.0 * 3.14159265358979323846 / 180.0;
    double tan_vertical = std::tan(fov_rad * 0.5);
    double tan_horizontal = tan_vertical * std::max(double(aspect), 0.0001);
    godot::Vector3 half = size * 0.5f;
    double fit_distance = 0.0;
    for (int xi = -1; xi <= 1; xi += 2) {
        for (int yi = -1; yi <= 1; yi += 2) {
            for (int zi = -1; zi <= 1; zi += 2) {
                godot::Vector3 corner = center + godot::Vector3(
                    half.x * xi, half.y * yi, half.z * zi);
                godot::Vector3 offset = corner - center;
                double x = std::abs(double(offset.dot(right)));
                double y = std::abs(double(offset.dot(camera_up)));
                double toward_camera = double(offset.dot(view_dir));
                double required = toward_camera + std::max(
                    x / std::max(tan_horizontal, 0.0001),
                    y / std::max(tan_vertical, 0.0001));
                fit_distance = std::max(fit_distance, required);
            }
        }
    }
    return std::max(fit_distance * double(margin), 0.75);
}

godot::Dictionary score_camera_receipt(const godot::Array &receipt,
                                       int width, int height) {
    int visible_count = 0;
    int clipped_count = 0;
    int min_visible_pixels = std::numeric_limits<int>::max();
    int total_visible_pixels = 0;
    int overlap_area = 0;
    std::vector<godot::Rect2i> rects;
    for (int i = 0; i < receipt.size(); i++) {
        if (receipt[i].get_type() != godot::Variant::DICTIONARY) continue;
        godot::Dictionary item = receipt[i];
        if (!(bool)item.get("visible", false)) continue;
        visible_count++;
        int pixels = int(item.get("visible_pixels", 0));
        min_visible_pixels = std::min(min_visible_pixels, pixels);
        total_visible_pixels += pixels;
        godot::Dictionary raw_rect = item.get("screen_rect", godot::Dictionary());
        godot::Rect2i rect(
            int(raw_rect.get("x", 0)), int(raw_rect.get("y", 0)),
            int(raw_rect.get("width", 0)), int(raw_rect.get("height", 0)));
        if (rect.position.x <= 1 || rect.position.y <= 1 ||
            rect.position.x + rect.size.x >= width - 1 ||
            rect.position.y + rect.size.y >= height - 1) {
            clipped_count++;
        }
        rects.push_back(rect);
    }
    for (size_t i = 0; i < rects.size(); i++) {
        for (size_t j = i + 1; j < rects.size(); j++) {
            godot::Rect2i intersection = rects[i].intersection(rects[j]);
            overlap_area += std::max(intersection.get_area(), 0);
        }
    }
    if (visible_count == 0) min_visible_pixels = 0;
    double score = double(visible_count) * 1.0e12 -
                   double(clipped_count) * 1.0e10 +
                   double(min_visible_pixels) * 1.0e5 +
                   double(total_visible_pixels) -
                   double(overlap_area) * 10.0;
    godot::Dictionary metrics;
    metrics["score"] = score;
    metrics["visible_count"] = visible_count;
    metrics["clipped_count"] = clipped_count;
    metrics["min_visible_pixels"] = min_visible_pixels;
    metrics["total_visible_pixels"] = total_visible_pixels;
    metrics["overlap_area"] = overlap_area;
    return metrics;
}

enum class SearchStage {
    CANDIDATES,
    FINAL_RGB,
};

struct CameraSearchJob {
    godot::SubViewport *viewport = nullptr;
    godot::Node *root = nullptr;
    godot::Camera3D *camera = nullptr;
    godot::Array subjects;
    godot::Vector3 center;
    godot::Vector3 size;
    godot::Vector2i original_viewport_size;
    godot::Transform3D original_transform;
    godot::Camera3D::ProjectionType original_projection =
        godot::Camera3D::PROJECTION_PERSPECTIVE;
    float original_fov = 70.0f;
    float original_size = 1.0f;
    float original_near = 0.05f;
    float original_far = 1000.0f;
    godot::SubViewport::UpdateMode original_update_mode =
        godot::SubViewport::UPDATE_ALWAYS;
    std::vector<MaterialState> materials;
    std::vector<CanvasVisibilityState> canvas;
    godot::Ref<godot::Environment> original_camera_environment;
    godot::Ref<godot::Environment> mask_environment;
    std::vector<godot::Ref<godot::ShaderMaterial>> subject_materials;
    godot::Ref<godot::ShaderMaterial> background_material;
    std::vector<Candidate> candidates;
    int candidate_index = 0;
    double best_score = -std::numeric_limits<double>::infinity();
    int best_index = -1;
    godot::Dictionary best_metrics;
    godot::Array best_receipt;
    double best_distance = 0.0;
    godot::Vector3 best_position;
    SearchStage stage = SearchStage::CANDIDATES;
    bool restored = false;
};

std::unique_ptr<CameraSearchJob> &camera_search_job() {
    static std::unique_ptr<CameraSearchJob> job;
    return job;
}

void apply_mask_materials(CameraSearchJob &job) {
    set_synthetic_background(job.canvas, true);
    job.camera->set_environment(job.mask_environment);
    for (const MaterialState &entry : job.materials) {
        if (!entry.geometry) continue;
        if (entry.subject_index >= 0 &&
            entry.subject_index < int(job.subject_materials.size())) {
            entry.geometry->set_material_override(
                job.subject_materials[entry.subject_index]);
        } else {
            entry.geometry->set_material_override(job.background_material);
        }
    }
}

void restore_search_job(CameraSearchJob &job) {
    if (job.restored) return;
    restore_materials(job.materials);
    set_synthetic_background(job.canvas, false);
    if (job.camera) {
        job.camera->set_environment(job.original_camera_environment);
        job.camera->set_transform(job.original_transform);
        job.camera->set_projection(job.original_projection);
        job.camera->set_fov(job.original_fov);
        job.camera->set_size(job.original_size);
        job.camera->set_near(job.original_near);
        job.camera->set_far(job.original_far);
    }
    if (job.viewport) {
        job.viewport->set_size(job.original_viewport_size);
        job.viewport->set_update_mode(job.original_update_mode);
    }
    job.restored = true;
}

void configure_candidate(CameraSearchJob &job, int candidate_index) {
    constexpr int search_width = 320;
    constexpr int search_height = 180;
    const Candidate &candidate = job.candidates[candidate_index];
    double distance = fit_perspective_distance(
        job.center, job.size, candidate.direction,
        float(search_width) / float(search_height), 1.15f);
    double radius = std::max(double(job.size.length()) * 0.5, 1.0);
    godot::Vector3 camera_position =
        job.center + candidate.direction * float(distance);
    job.camera->set_far(
        float(std::max(distance + radius * 10.0, 1000.0)));
    job.camera->look_at_from_position(
        camera_position, job.center, godot::Vector3(0, 1, 0));
    job.viewport->set_update_mode(godot::SubViewport::UPDATE_ONCE);
}

godot::Array scale_receipt(const godot::Array &receipt,
                           const godot::Vector2i &output_size) {
    godot::Array scaled;
    const double scale_x = double(output_size.x) / 320.0;
    const double scale_y = double(output_size.y) / 180.0;
    for (int i = 0; i < receipt.size(); i++) {
        godot::Dictionary item =
            godot::Dictionary(receipt[i]).duplicate();
        if ((bool)item.get("visible", false)) {
            godot::Dictionary rect = item.get(
                "screen_rect", godot::Dictionary());
            rect["x"] = int(std::round(int(rect.get("x", 0)) * scale_x));
            rect["y"] = int(std::round(int(rect.get("y", 0)) * scale_y));
            rect["width"] = int(std::round(
                int(rect.get("width", 0)) * scale_x));
            rect["height"] = int(std::round(
                int(rect.get("height", 0)) * scale_y));
            item["screen_rect"] = rect;
            item["visible_pixels"] = int(std::round(
                int(item.get("visible_pixels", 0)) * scale_x * scale_y));
        }
        scaled.append(item);
    }
    return scaled;
}

void mark_pending(godot::Dictionary &result) {
    result["success"] = true;
    result["pending"] = true;
}

} // namespace

void FennaraScreenshotSceneTool::_reset_camera_search_job() {
    std::unique_ptr<CameraSearchJob> &job = camera_search_job();
    if (job) {
        restore_search_job(*job);
        job.reset();
    }
}

godot::Ref<godot::Image> FennaraScreenshotSceneTool::_capture_camera_searched_3d(
    godot::SubViewport *viewport,
    godot::Dictionary &result) {
    std::unique_ptr<CameraSearchJob> &job = camera_search_job();
    if (!job) {
        godot::Dictionary state = _camera_search_capture_state_ref();
        godot::Object *root_object = state.get("root", godot::Variant());
        godot::Object *camera_object = state.get("camera", godot::Variant());
        godot::Node *root = godot::Object::cast_to<godot::Node>(root_object);
        godot::Camera3D *camera =
            godot::Object::cast_to<godot::Camera3D>(camera_object);
        godot::Array subjects = state.get("capture_nodes", godot::Array());
        if (!viewport || !root || !camera || subjects.is_empty()) {
            result["error"] =
                "Camera-searched 3D capture state was incomplete";
            return godot::Ref<godot::Image>();
        }

        job = std::make_unique<CameraSearchJob>();
        job->viewport = viewport;
        job->root = root;
        job->camera = camera;
        job->subjects = subjects;
        job->center = state.get("bounds_center", godot::Vector3());
        job->size = state.get("bounds_size", godot::Vector3(2, 2, 2));
        job->original_viewport_size = viewport->get_size();
        job->original_transform = camera->get_transform();
        job->original_projection = camera->get_projection();
        job->original_fov = camera->get_fov();
        job->original_size = camera->get_size();
        job->original_near = camera->get_near();
        job->original_far = camera->get_far();
        job->original_update_mode = viewport->get_update_mode();

        collect_render_state(
            viewport, subjects, job->materials, job->canvas);
        job->original_camera_environment = camera->get_environment();
        job->mask_environment.instantiate();
        job->mask_environment->set_background(godot::Environment::BG_COLOR);
        job->mask_environment->set_bg_color(
            godot::Color(0.005, 0.006, 0.009, 1.0));
        job->mask_environment->set_ambient_source(
            godot::Environment::AMBIENT_SOURCE_COLOR);
        job->mask_environment->set_ambient_light_color(
            godot::Color(0.0, 0.0, 0.0, 1.0));
        job->mask_environment->set_ambient_light_energy(0.0f);
        for (int i = 0;
             i < std::min(int(subjects.size()), SUBJECT_COLOR_COUNT); i++) {
            job->subject_materials.push_back(
                make_flat_material(subject_color(i)));
        }
        job->background_material =
            make_flat_material(godot::Color(0.015, 0.017, 0.022, 1.0));

        const int elevations[] = {18, 38};
        for (int elevation : elevations) {
            for (int azimuth = 0; azimuth < 360; azimuth += 45) {
                double azimuth_rad =
                    double(azimuth) * 3.14159265358979323846 / 180.0;
                double elevation_rad =
                    double(elevation) * 3.14159265358979323846 / 180.0;
                godot::Vector3 direction(
                    float(std::cos(elevation_rad) * std::cos(azimuth_rad)),
                    float(std::sin(elevation_rad)),
                    float(std::cos(elevation_rad) * std::sin(azimuth_rad)));
                job->candidates.push_back(
                    {azimuth, elevation, direction.normalized()});
            }
        }
        job->candidates.push_back(
            {0, 70,
             godot::Vector3(
                 float(std::cos(70.0 * 3.14159265358979323846 / 180.0)),
                 float(std::sin(70.0 * 3.14159265358979323846 / 180.0)),
                 0)
                 .normalized()});

        viewport->set_size(godot::Vector2i(320, 180));
        camera->set_projection(godot::Camera3D::PROJECTION_PERSPECTIVE);
        camera->set_fov(70.0f);
        camera->set_near(0.05f);
        apply_mask_materials(*job);
        configure_candidate(*job, 0);
        mark_pending(result);
        return godot::Ref<godot::Image>();
    }

    if (job->viewport != viewport || !job->camera || !job->root) {
        _reset_camera_search_job();
        result["error"] = "Camera search job lost its detached scene";
        return godot::Ref<godot::Image>();
    }

    godot::Ref<godot::ViewportTexture> texture = viewport->get_texture();
    if (texture.is_null()) {
        _reset_camera_search_job();
        result["error"] = "Could not get camera-search viewport texture";
        return godot::Ref<godot::Image>();
    }

    if (job->stage == SearchStage::CANDIDATES) {
        godot::Ref<godot::Image> mask = texture->get_image();
        if (mask.is_null()) {
            _reset_camera_search_job();
            result["error"] = "Could not read a camera-search mask";
            return godot::Ref<godot::Image>();
        }
        godot::Array receipt =
            measure_subjects(mask, job->root, job->subjects);
        godot::Dictionary metrics = score_camera_receipt(receipt, 320, 180);
        double score = double(metrics.get("score", -1.0e30));
        if (score > job->best_score) {
            job->best_score = score;
            job->best_index = job->candidate_index;
            job->best_metrics = metrics;
            job->best_receipt = receipt;
        }

        job->candidate_index++;
        if (job->candidate_index < int(job->candidates.size())) {
            configure_candidate(*job, job->candidate_index);
            mark_pending(result);
            return godot::Ref<godot::Image>();
        }
        if (job->best_index < 0) {
            _reset_camera_search_job();
            result["error"] =
                "Deterministic camera search produced no usable candidate";
            return godot::Ref<godot::Image>();
        }

        const Candidate &best = job->candidates[job->best_index];
        viewport->set_size(job->original_viewport_size);
        float output_aspect = job->original_viewport_size.y > 0
            ? float(job->original_viewport_size.x) /
                  float(job->original_viewport_size.y)
            : 1.0f;
        job->best_distance = fit_perspective_distance(
            job->center, job->size, best.direction, output_aspect, 1.15f);
        double radius = std::max(double(job->size.length()) * 0.5, 1.0);
        job->best_position =
            job->center + best.direction * float(job->best_distance);
        job->camera->set_far(float(std::max(
            job->best_distance + radius * 10.0, 1000.0)));
        job->camera->look_at_from_position(
            job->best_position, job->center, godot::Vector3(0, 1, 0));
        restore_materials(job->materials);
        set_synthetic_background(job->canvas, false);
        job->camera->set_environment(job->original_camera_environment);
        job->stage = SearchStage::FINAL_RGB;
        viewport->set_update_mode(godot::SubViewport::UPDATE_ONCE);
        mark_pending(result);
        return godot::Ref<godot::Image>();
    }

    godot::Ref<godot::Image> chosen_rgb = texture->get_image();
    if (chosen_rgb.is_null()) {
        _reset_camera_search_job();
        result["error"] = "Could not render the selected camera candidate";
        return godot::Ref<godot::Image>();
    }

    const Candidate best = job->candidates[job->best_index];
    const int candidate_count = int(job->candidates.size());
    const int best_index = job->best_index;
    const double best_score = job->best_score;
    const double best_distance = job->best_distance;
    const godot::Vector3 best_position = job->best_position;
    const int selected_count = job->subjects.size();
    godot::Array visibility = scale_receipt(
        job->best_receipt,
        godot::Vector2i(chosen_rgb->get_width(), chosen_rgb->get_height()));
    godot::Dictionary final_metrics = score_camera_receipt(
        visibility, chosen_rgb->get_width(), chosen_rgb->get_height());

    restore_search_job(*job);
    job.reset();

    godot::Dictionary search_receipt;
    search_receipt["candidate_count"] = candidate_count;
    search_receipt["chosen_index"] = best_index;
    search_receipt["chosen_azimuth_degrees"] = best.azimuth;
    search_receipt["chosen_elevation_degrees"] = best.elevation;
    search_receipt["chosen_direction"] = best.direction;
    search_receipt["chosen_camera_position"] = best_position;
    search_receipt["selected_count"] = selected_count;
    search_receipt["visible_count"] = final_metrics.get("visible_count", 0);
    search_receipt["clipped_count"] = final_metrics.get("clipped_count", 0);
    search_receipt["all_selected_visible"] =
        int(final_metrics.get("visible_count", 0)) == selected_count;
    search_receipt["search_score"] = best_score;
    search_receipt["final_metrics"] = final_metrics;
    search_receipt["measurement_viewport"] = godot::Vector2i(320, 180);

    result["selected_node_visibility"] = visibility;
    result["camera_search"] = search_receipt;
    if (!(bool)search_receipt["all_selected_visible"]) {
        result["camera_search_warning"] =
            "No tested camera made every selected node visible.";
    }
    result["view"] = "camera_search";
    result["projection"] = "perspective";
    result["camera_position"] = best_position;
    result["camera_distance"] = best_distance;
    result["image_role"] = "camera_searched_rgb";
    return chosen_rgb;
}

} // namespace fennara
