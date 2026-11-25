#pragma once

#include <string>
#include <vector>

namespace flask {

struct Package {
    std::string name;
    std::string version;
    std::string backend;
};


class Resolver {
public:
    Resolver();
    ~Resolver() = default;

    std::string resolve(const std::string& package_name) const;

    std::vector<std::string> available_backends() const;

private:
    bool backend_available(const std::string& bin) const;
};

} // namespace flask
