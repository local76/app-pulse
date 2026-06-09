# pulse

> A local, real-time system resource monitor.

`pulse` is a single-binary TUI for live CPU, GPU, memory, disk, and network telemetry. Multi-pane Ratatui dashboard. Runs on Windows and Linux with no admin elevation, no cloud calls, no telemetry.

`pulse` is part of the [local76](https://github.com/local76/local76) ecosystem and depends on [`library`](https://github.com/local76/library) for its TUI widgets and design system.

---

## Features

- **Live CPU/GPU/memory/disk/network panels.** All sampled at 1-second intervals, throttled in memory.
- **Multi-pane TUI.** Each resource gets its own pane. Tab through them. Resize the terminal freely.
- **Process list.** Sortable by PID, name, CPU%, memory%.
- **GPU support on Windows.** Reads NVIDIA / AMD / Intel GPU stats via the local driver.
- **Hot-loop caching.** Zero per-frame FFI/registry reads.

---

## Install

### Windows
- **Standalone**: download `pulse.exe` from the [latest release](https://github.com/local76/pulse/releases).
- **winget**: `winget install local76.pulse`
- **MSI**: download the `.msi` from the releases page.

### Linux
- **Debian/Ubuntu**: `sudo dpkg -i pulse.deb`
- **Red Hat/Fedora**: `sudo rpm -i pulse.rpm`
- **Arch (AUR)**: `yay -S pulse-bin`

---

## Usage

```
pulse                      # launch the live monitor
pulse --no-live            # one-shot snapshot, suitable for shell profiles
pulse --panes cpu,gpu,net  # choose which panes to show
pulse --refresh 500ms      # override the 1-second refresh interval
pulse --version
pulse --help
```

---

## Configuration

A YAML config file is auto-generated on first run:

- **Windows**: `%APPDATA%\pulse\config.yaml`
- **Linux**: `~/.config/pulse/config.yaml`

---

## Build from source

```pwsh
git clone https://github.com/local76/pulse.git
cd pulse
cargo build --release
```

---

## License

MIT. See [LICENSE.md](LICENSE.md).
