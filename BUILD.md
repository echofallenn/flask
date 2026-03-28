# building flask from source

## requirements

| tool | version | why |
|------|---------|-----|
| Rust + Cargo | ≥ 1.75 | core language |
| GCC or Clang | ≥ 11 | compiles the C++ resolver |
| CMake | ≥ 3.16 | optional, used if you build C++ separately |
| make | any | convenience wrapper |

On Arch:
```sh
sudo pacman -S rust gcc make cmake
```

On Ubuntu/Debian:
```sh
sudo apt install cargo gcc g++ make cmake
```
<sup> obs: you can only use flatpak and snap on non-arch based distros, for obvious reasons <sup>

---

## quick build

```sh
git clone https://github.com/echofallenn/flask
cd flask
cargo build
```

The `build.rs` script automatically compiles `cpp/resolver.cpp` into a static library and links it into the binary. You don't need to run CMake manually.

---

## release build

```sh
cargo build --release
# binary at: target/release/flask
```

---

## install system-wide

```sh
make install
# copies target/release/flask to /usr/local/bin/flask
```

Or manually:
```sh
sudo install -m 755 target/release/flask /usr/local/bin/flask
```

---

## optional: build C++ separately with CMake

If you want to iterate on the C++ resolver without rebuilding the whole Rust crate:

```sh
cmake -B build -S .
cmake --build build
# produces build/libresolver.a
```

Then point cargo to it by setting `FLASK_RESOLVER_LIB`:
```sh
FLASK_RESOLVER_LIB=build/libresolver.a cargo build
```

---

## run tests

```sh
cargo test
```

Tests skip automatically if the relevant backend binary isn't installed.

---

## project layout

```
flask/
├── src/
│   ├── main.rs          # entry point
│   ├── cli.rs           # clap CLI definition
│   └── manager/
│       ├── mod.rs       # PackageManager trait + backend enum
│       ├── snap.rs
│       ├── flatpak.rs
│       ├── pacman.rs
│       └── aur.rs
├── bridge/
│   ├── ffi.rs           # Rust → C FFI exports
│   └── bindings.h       # C header for C++ to include
├── cpp/
│   ├── resolver.hpp
│   └── resolver.cpp     # C++ dependency resolver
├── tests/               # integration tests per backend
├── config/
│   └── flask.toml       # default config
├── build.rs             # cargo build script (compiles C++)
├── CMakeLists.txt       # optional standalone C++ build
└── Makefile             # convenience targets
```

---

## troubleshooting

**`cc` crate fails to find g++`**
Make sure `g++` is installed and in PATH. On Arch: `sudo pacman -S gcc`.

**AUR helper not found**
Install `yay` or `paru` before using `--backend aur`.

**`flask_which` undefined symbol**
The FFI bridge didn't link correctly. Run `cargo clean && cargo build` to force a full rebuild.
