# flask

**your totally unnecessary multi-repo package manager**

`flask` is a unified CLI for managing packages across `snap`, `flatpak`, `pacman`, and the `AUR`, because apparently one package manager was never enough.

Written in Rust with a C++ dependency resolver. Completely over-engineered. Absolutely worth it.

---

## features

- single command for snap, flatpak, pacman and AUR
- auto-detects available backends on your system
- C++ resolver figures out which backend owns a package
- async under the hood (tokio)
- colored output because life is short

---

## usage

```sh
# install a package (uses default backend)
flask install neovim

# use a specific backend
flask flatpak install com.spotify.Client
flask snap install vlc
flask aur install paru-bin

# search across a backend
flask --backend pacman search neovim

# update everything
flask update

# list installed
flask list

# package info
flask info neovim

# remove
flask remove neovim
```

---

## backends

| backend | binary required | notes |
|---------|----------------|-------|
| pacman  | `pacman`       | default on Arch |
| aur     | `yay` or `paru`| prefers yay |
| flatpak | `flatpak`      | any distro |
| snap    | `snap`         | any distro |

---

## build from source

See [BUILD.md](BUILD.md).

---

## configuration

Copy `config/flask.toml` to `~/.config/flask/flask.toml` and edit as needed:

```toml
[general]
default_backend = "pacman"
confirm = true

[backends]
aur.helper = "yay"
```

---

## license

[LICENSE](LICENSE)
