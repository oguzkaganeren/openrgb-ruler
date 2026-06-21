# OpenRGB Ruler

A **GTK4** (Rust + relm4) desktop application that lets you define **trigger → RGB action** rules for automated RGB lighting control via [OpenRGB](https://openrgb.org/).

Replace manual shell scripts with a GUI-configurable rules engine — lock your screen, launch a game, or go idle, and your RGB lighting responds automatically.

## Screenshots

<img src="screens/image.png" alt="Empty Rules List" width="400">

<img src="screens/image2.png" alt="Rule Editor — System Lock / Turn Off" width="400">

<img src="screens/image3.png" alt="Rule Editor — Time of Day / Set Color" width="400">

## Features

- **Visual rules editor** — create, edit, reorder, and delete trigger→action rules without touching the terminal
- **Multiple trigger types**:
  - Screen lock / unlock (via DBus)
  - Process start / stop (polls `/proc`)
  - Session idle timeout
  - Time of day (cron-style)
- **Multiple RGB actions**:
  - Turn off all LEDs
  - Set a solid color (hex picker)
  - Load an OpenRGB profile
  - Set brightness (via OpenRGB SDK)
- **System tray** — the rules engine keeps running in the background even when the window is closed
- **Auto-start** — optionally launch on login
- **Live status bar** — shows OpenRGB connection state

## Requirements

- Linux (KDE or GNOME recommended for DBus events)
- GTK4 libraries (`gtk4` package)
- [`openrgb`](https://openrgb.org/) binary in `PATH`
- OpenRGB server running (for profile/SDK actions)

## Installation

### Arch Linux (AUR)

Available on the AUR:

- **[openrgb-ruler](https://aur.archlinux.org/packages/openrgb-ruler)** — stable release
- **openrgb-ruler-git** — latest from `main`

```bash
# Stable release (recommended)
yay -S openrgb-ruler
```

### Build from Source

```bash
git clone https://github.com/oguzkaganeren/openrgb-ruler
cd openrgb-ruler

# Debug build
cargo build

# Release build
cargo build --release

# Run
./target/release/openrgb-ruler-gtk
```

Requires: Rust toolchain (`rustup`), GTK4 development libraries (`gtk4` + `pkg-config`).

## Quick Start

1. Start **OpenRGB** and make sure it's running in server mode (`openrgb --server`).
2. Launch **OpenRGB Ruler**.
3. Click **+** to add a rule, choose a trigger (e.g. *Screen Lock*) and an action (e.g. *Turn Off*).
4. Enable the rule and close the window — the engine keeps running in the tray.

## Rules Config

Rules are stored at `~/.config/openrgb-ruler/rules.json`. You can back them up or sync them across machines manually.

## Stack

| Layer | Technology |
|-------|-----------|
| UI | GTK4 + relm4 |
| Language | Rust |
| RGB control | OpenRGB CLI / SDK (port 6742) |
| System events | zbus (DBus), `/proc` polling |
| System tray | ksni (StatusNotifierItem) |
| Async runtime | Tokio |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

GNU General Public License v3.0 — see [LICENSE.md](LICENSE.md).
