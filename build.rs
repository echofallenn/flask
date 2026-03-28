use std::process::Command;
use std::path::Path;

fn main() {
    let cpp_dir = Path::new("cpp");

    // Compile resolver.cpp into a static lib using cc crate
    cc::Build::new()
        .cpp(true)
        .file(cpp_dir.join("resolver.cpp"))
        .include(cpp_dir)
        .flag("-std=c++17")
        .flag("-O2")
        .compile("resolver");

    println!("cargo:rerun-if-changed=cpp/resolver.cpp");
    println!("cargo:rerun-if-changed=cpp/resolver.hpp");
    println!("cargo:rerun-if-changed=bridge/bindings.h");
}
