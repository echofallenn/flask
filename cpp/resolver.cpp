#include "resolver.hpp"
#include "../bridge/bindings.h"

#include <array>
#include <cstdlib>

namespace flask {

static const std::array<std::string, 4> BACKENDS = {
    "pacman", "yay", "flatpak", "snap"
};

Resolver::Resolver() {}

bool Resolver::backend_available(const std::string& bin) const {
    return flask_which(bin.c_str()) == 1;
}

std::vector<std::string> Resolver::available_backends() const {
    std::vector<std::string> result;
    for (const auto& b : BACKENDS) {
        if (backend_available(b)) {
            result.push_back(b);
        }
    }
    return result;
}

std::string Resolver::resolve(const std::string& package_name) const {
    (void)package_name; // future: query each backend's DB
    auto backends = available_backends();
    if (backends.empty()) return "";
    return backends[0];
}

} // namespace flask

// C-compatible entry points for Rust FFI
extern "C" {

const char* flask_resolve_backend(const char* package_name) {
    static flask::Resolver resolver;
    static std::string result;
    result = resolver.resolve(package_name ? package_name : "");
    return result.c_str();
}

} // extern "C"
