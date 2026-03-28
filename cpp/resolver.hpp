#pragma once

#include <string>
#include <vector>

namespace flask {

struct Package {
    std::string name;
    std::string version;
    std::string backend;
};

/// Resolves which backend owns a package name.
/// Checks availability of snap/flatpak/pacman/aur in that order.
class Resolver {
public:
    Resolver();
    ~Resolver() = default;

    /// Returns the preferred backend for a given package name.
    /// Returns empty string if none found.
    std::string resolve(const std::string& package_name) const;

    /// Returns all backends currently available on the system.
    std::vector<std::string> available_backends() const;

private:
    bool backend_available(const std::string& bin) const;
};

} // namespace flask
