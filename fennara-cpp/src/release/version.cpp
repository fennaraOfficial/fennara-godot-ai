#include "fennara/release/version.hpp"

#include <cstdint>
#include <limits>
#include <string>
#include <vector>

namespace fennara::release_version {
namespace {

struct Identifier {
    bool numeric = false;
    uint64_t number = 0;
    std::string text;
};

struct Version {
    uint64_t major = 0;
    uint64_t minor = 0;
    uint64_t patch = 0;
    std::vector<Identifier> prerelease;
};

bool valid_identifier_character(char character) {
    return (character >= '0' && character <= '9') ||
           (character >= 'A' && character <= 'Z') ||
           (character >= 'a' && character <= 'z') || character == '-';
}

bool parse_number(const std::string &value, uint64_t &number) {
    if (value.empty() || (value.size() > 1 && value.front() == '0')) {
        return false;
    }
    uint64_t parsed = 0;
    for (char character : value) {
        if (character < '0' || character > '9') {
            return false;
        }
        const uint64_t digit = static_cast<uint64_t>(character - '0');
        if (parsed > (std::numeric_limits<uint64_t>::max() - digit) / 10) {
            return false;
        }
        parsed = parsed * 10 + digit;
    }
    number = parsed;
    return true;
}

std::vector<std::string> split(const std::string &value, char delimiter) {
    std::vector<std::string> parts;
    size_t start = 0;
    while (true) {
        const size_t end = value.find(delimiter, start);
        parts.push_back(value.substr(start, end == std::string::npos ? end : end - start));
        if (end == std::string::npos) {
            return parts;
        }
        start = end + 1;
    }
}

std::optional<Version> parse(godot::String input) {
    const std::string raw = input.utf8().get_data();
    if (raw.empty()) {
        return std::nullopt;
    }

    const size_t plus = raw.find('+');
    if (plus != std::string::npos) {
        return std::nullopt;
    }
    const std::string &without_build = raw;
    const size_t dash = without_build.find('-');
    const std::string core = without_build.substr(0, dash);
    const std::vector<std::string> core_parts = split(core, '.');
    if (core_parts.size() != 3) {
        return std::nullopt;
    }

    Version version;
    if (!parse_number(core_parts[0], version.major) ||
        !parse_number(core_parts[1], version.minor) ||
        !parse_number(core_parts[2], version.patch)) {
        return std::nullopt;
    }

    if (dash == std::string::npos) {
        return version;
    }
    const std::string prerelease = without_build.substr(dash + 1);
    if (prerelease.empty()) {
        return std::nullopt;
    }
    for (const std::string &part : split(prerelease, '.')) {
        if (part.empty()) {
            return std::nullopt;
        }
        bool numeric = true;
        for (char character : part) {
            if (!valid_identifier_character(character)) {
                return std::nullopt;
            }
            numeric = numeric && character >= '0' && character <= '9';
        }
        Identifier identifier;
        identifier.numeric = numeric;
        identifier.text = part;
        if (numeric && !parse_number(part, identifier.number)) {
            return std::nullopt;
        }
        version.prerelease.push_back(identifier);
    }
    return version;
}

int compare_number(uint64_t left, uint64_t right) {
    return left < right ? -1 : left > right ? 1 : 0;
}

int compare_prerelease(const std::vector<Identifier> &left,
                       const std::vector<Identifier> &right) {
    if (left.empty() || right.empty()) {
        return left.empty() == right.empty() ? 0 : left.empty() ? 1 : -1;
    }
    const size_t count = left.size() < right.size() ? left.size() : right.size();
    for (size_t index = 0; index < count; index++) {
        const Identifier &left_identifier = left[index];
        const Identifier &right_identifier = right[index];
        if (left_identifier.numeric && right_identifier.numeric) {
            const int result = compare_number(left_identifier.number, right_identifier.number);
            if (result != 0) {
                return result;
            }
            continue;
        }
        if (left_identifier.numeric != right_identifier.numeric) {
            return left_identifier.numeric ? -1 : 1;
        }
        if (left_identifier.text != right_identifier.text) {
            return left_identifier.text < right_identifier.text ? -1 : 1;
        }
    }
    return compare_number(left.size(), right.size());
}

} // namespace

godot::String normalize(godot::String version) {
    version = version.strip_edges();
    if (version.begins_with("v")) {
        version = version.substr(1);
    }
    return version;
}

bool is_valid(const godot::String &version) {
    return parse(version).has_value();
}

std::optional<int> compare(const godot::String &left, const godot::String &right) {
    const std::optional<Version> left_version = parse(left);
    const std::optional<Version> right_version = parse(right);
    if (!left_version.has_value() || !right_version.has_value()) {
        return std::nullopt;
    }
    int result = compare_number(left_version->major, right_version->major);
    if (result == 0) {
        result = compare_number(left_version->minor, right_version->minor);
    }
    if (result == 0) {
        result = compare_number(left_version->patch, right_version->patch);
    }
    if (result == 0) {
        result = compare_prerelease(left_version->prerelease, right_version->prerelease);
    }
    return result;
}

} // namespace fennara::release_version
