#include "fennara/app_paths.hpp"
#include "fennara/setup/first_run_setup.hpp"

#include "fennara/release/identity.hpp"
#include "fennara/release/version.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/hashing_context.hpp>
#include <godot_cpp/classes/http_client.hpp>
#include <godot_cpp/classes/http_request.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/zip_reader.hpp>

namespace fennara {
namespace {

bool valid_asset_name(const godot::String &name) {
    return !name.is_empty() && name.get_file() == name && !name.contains("/") &&
           !name.contains("\\");
}

bool valid_sha256(const godot::String &value) {
    if (value.length() != 64) {
        return false;
    }
    for (int i = 0; i < value.length(); i++) {
        const char32_t c = value[i];
        const bool digit = c >= '0' && c <= '9';
        const bool lower = c >= 'a' && c <= 'f';
        const bool upper = c >= 'A' && c <= 'F';
        if (!digit && !lower && !upper) {
            return false;
        }
    }
    return true;
}

} // namespace

void FirstRunSetup::_on_manifest_request_completed(int64_t result, int64_t response_code,
                                                   godot::PackedStringArray headers,
                                                   godot::PackedByteArray body) {
    (void)headers;
    if (step != Step::DownloadingManifest) {
        return;
    }
    if (result != godot::HTTPRequest::RESULT_SUCCESS || response_code != 200) {
        _fail("FEN-SETUP-MANIFEST-DOWNLOAD", "The matching release manifest could not be "
                                             "downloaded. Check your connection and retry.");
        return;
    }

    const godot::Variant parsed = godot::JSON::parse_string(body.get_string_from_utf8());
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        _fail("FEN-SETUP-MANIFEST-INVALID", "The release manifest is not valid JSON.");
        return;
    }
    const godot::Dictionary manifest = parsed;
    if ((int64_t)manifest.get("schema_version", 0) != 1 ||
        godot::String(manifest.get("version", "")) != addon_version) {
        _fail("FEN-SETUP-MANIFEST-INVALID",
              "The release manifest does not match this addon version.");
        return;
    }
    godot::String identity_error;
    if (!release_identity::manifest_matches(manifest, addon_identity, identity_error)) {
        _fail("FEN-SETUP-MANIFEST-IDENTITY", identity_error);
        return;
    }
    const godot::String minimum_cli_version = manifest.get("minimum_cli_version", "");
    if (!release_version::is_valid(minimum_cli_version) ||
        release_version::compare(addon_version, minimum_cli_version).value_or(-1) < 0) {
        _fail("FEN-SETUP-MANIFEST-INVALID",
              "The release manifest has an incompatible minimum CLI version.");
        return;
    }

    const godot::Variant assets_value = manifest.get("assets", godot::Dictionary());
    if (assets_value.get_type() != godot::Variant::DICTIONARY) {
        _fail("FEN-SETUP-MANIFEST-INVALID", "The release manifest is missing its assets.");
        return;
    }
    const godot::Dictionary assets = assets_value;
    const godot::Variant cli_value = assets.get("cli", godot::Dictionary());
    if (cli_value.get_type() != godot::Variant::DICTIONARY) {
        _fail("FEN-SETUP-MANIFEST-INVALID", "The release manifest is missing CLI assets.");
        return;
    }
    const godot::Dictionary cli_assets = cli_value;
    const godot::String platform_key = _platform_key();
    const godot::Variant asset_value = cli_assets.get(platform_key, godot::Dictionary());
    if (platform_key.is_empty() || asset_value.get_type() != godot::Variant::DICTIONARY) {
        _fail("FEN-SETUP-PLATFORM-UNSUPPORTED",
              "This release does not provide a CLI for the current platform and architecture.");
        return;
    }

    const godot::Dictionary asset = asset_value;
    cli_asset_name = asset.get("name", "");
    expected_cli_sha256 = godot::String(asset.get("sha256", "")).to_lower();
    const godot::String expected_cli_name =
        "fennara-cli-" + platform_key + "-v" + addon_version + ".zip";
    if (!valid_asset_name(cli_asset_name) || cli_asset_name != expected_cli_name ||
        !valid_sha256(expected_cli_sha256)) {
        _fail("FEN-SETUP-MANIFEST-INVALID", "The selected CLI asset metadata is invalid.");
        return;
    }
    if (_test_failure("cli_download")) {
        _fail("FEN-SETUP-CLI-DOWNLOAD", "Simulated CLI download failure.");
        return;
    }

    step = Step::DownloadingCli;
    _set_status("Downloading the Fennara CLI...", cli_asset_name);
    cli_request->set_download_file(cli_archive_path);
    const godot::Error request_error =
        cli_request->request(_release_asset_url(cli_asset_name),
                             godot::PackedStringArray{"Accept: application/octet-stream",
                                                      "User-Agent: fennara-godot-setup"},
                             godot::HTTPClient::METHOD_GET);
    if (request_error != godot::OK) {
        _fail("FEN-SETUP-CLI-DOWNLOAD", "Could not start the CLI download.");
    }
}

void FirstRunSetup::_on_cli_request_completed(int64_t result, int64_t response_code,
                                              godot::PackedStringArray headers,
                                              godot::PackedByteArray body) {
    (void)headers;
    (void)body;
    if (step != Step::DownloadingCli) {
        return;
    }
    if (result != godot::HTTPRequest::RESULT_SUCCESS || response_code != 200 ||
        !godot::FileAccess::file_exists(cli_archive_path)) {
        _fail("FEN-SETUP-CLI-DOWNLOAD",
              "The Fennara CLI could not be downloaded. Check your connection and retry.");
        return;
    }

    step = Step::LaunchingInstaller;
    _set_status("Verifying the Fennara CLI...", "Checking the release SHA-256 hash");
    if (_test_failure("hash")) {
        expected_cli_sha256 = godot::String("0").repeat(64);
    }
    if (!_install_verified_cli()) {
        return;
    }
    _set_status("Starting the Fennara installer...",
                "The CLI will install all matching components");
    _launch_installer();
}

bool FirstRunSetup::_install_verified_cli() {
    const godot::PackedByteArray archive = godot::FileAccess::get_file_as_bytes(cli_archive_path);
    if (archive.is_empty()) {
        _fail("FEN-SETUP-CLI-DOWNLOAD", "The downloaded CLI archive is empty or unreadable.");
        return false;
    }

    godot::Ref<godot::HashingContext> hashing;
    hashing.instantiate();
    if (hashing->start(godot::HashingContext::HASH_SHA256) != godot::OK ||
        hashing->update(archive) != godot::OK) {
        _fail("FEN-SETUP-CLI-VERIFY", "Fennara could not hash the downloaded CLI archive.");
        return false;
    }
    const godot::String actual_sha256 = hashing->finish().hex_encode().to_lower();
    if (actual_sha256 != expected_cli_sha256) {
        _fail("FEN-SETUP-CLI-HASH-MISMATCH",
              "The downloaded CLI did not match the release hash. Nothing was installed.");
        return false;
    }

    godot::Ref<godot::ZIPReader> zip;
    zip.instantiate();
    if (zip->open(cli_archive_path) != godot::OK) {
        _fail("FEN-SETUP-CLI-ARCHIVE", "The verified CLI archive could not be opened.");
        return false;
    }
    const godot::String entry = _cli_archive_entry();
    if (!zip->get_files().has(entry)) {
        zip->close();
        _fail("FEN-SETUP-CLI-ARCHIVE", "The verified archive does not contain the Fennara CLI.");
        return false;
    }
    const godot::PackedByteArray executable = zip->read_file(entry);
    zip->close();
    if (executable.is_empty()) {
        _fail("FEN-SETUP-CLI-ARCHIVE", "The CLI executable in the verified archive is empty.");
        return false;
    }

    const godot::String target = app_paths::cli_binary_path();
    if (godot::DirAccess::make_dir_recursive_absolute(target.get_base_dir()) != godot::OK) {
        _fail("FEN-SETUP-STAGE-FILESYSTEM", "Fennara could not create its CLI directory.");
        return false;
    }

    const godot::String staged = target + godot::String(".setup");
    const godot::String backup = target + godot::String(".previous");
    godot::DirAccess::remove_absolute(staged);
    godot::DirAccess::remove_absolute(backup);
    godot::Ref<godot::FileAccess> file = godot::FileAccess::open(staged, godot::FileAccess::WRITE);
    if (file.is_null() || !file->store_buffer(executable)) {
        _fail("FEN-SETUP-STAGE-FILESYSTEM", "Fennara could not stage the verified CLI.");
        return false;
    }
    file->flush();
    file.unref();

    godot::OS *os = godot::OS::get_singleton();
    if (os != nullptr && os->get_name() != "Windows") {
        const uint64_t permissions =
            godot::FileAccess::UNIX_READ_OWNER | godot::FileAccess::UNIX_WRITE_OWNER |
            godot::FileAccess::UNIX_EXECUTE_OWNER | godot::FileAccess::UNIX_READ_GROUP |
            godot::FileAccess::UNIX_EXECUTE_GROUP | godot::FileAccess::UNIX_READ_OTHER |
            godot::FileAccess::UNIX_EXECUTE_OTHER;
        if (godot::FileAccess::set_unix_permissions(staged, permissions) != godot::OK) {
            godot::DirAccess::remove_absolute(staged);
            _fail("FEN-SETUP-STAGE-FILESYSTEM",
                  "Fennara could not make the staged CLI executable.");
            return false;
        }
    }

    if (godot::FileAccess::file_exists(target) &&
        godot::DirAccess::rename_absolute(target, backup) != godot::OK) {
        godot::DirAccess::remove_absolute(staged);
        _fail("FEN-SETUP-STAGE-FILESYSTEM", "Fennara could not preserve the previous CLI.");
        return false;
    }
    if (godot::DirAccess::rename_absolute(staged, target) != godot::OK) {
        if (godot::FileAccess::file_exists(backup)) {
            godot::DirAccess::rename_absolute(backup, target);
        }
        godot::DirAccess::remove_absolute(staged);
        _fail("FEN-SETUP-STAGE-FILESYSTEM", "Fennara could not activate the verified CLI.");
        return false;
    }
    return true;
}

} // namespace fennara
