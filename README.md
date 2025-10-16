# Apple Music Web App

A desktop application for Apple Music built with Tauri.

## About

This is a lightweight desktop wrapper for Apple Music's web interface, providing a native application experience on Linux, macOS, and Windows.

## Prerequisites

### Arch Linux / Manjaro

```bash
# Install required dependencies including DRM support
sudo pacman -S webkit2gtk base-devel curl wget file openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly gst-libav
```

**Important for DRM/Audio Quality:** Install GStreamer plugins for full codec support and DRM (required for Apple Music protected content):

```bash
sudo pacman -S gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly gst-libav
```

### Ubuntu / Debian

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### Fedora

```bash
sudo dnf install webkit2gtk4.0-devel openssl-devel curl wget file gcc-c++ gtk3-devel libappindicator-gtk3-devel librsvg2-devel
sudo dnf group install "C Development Tools and Libraries"
```

### Other Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Tauri prerequisites](https://tauri.app/v2/guides/prerequisites/) for your platform

## Development

```bash
# Run in development mode
cargo tauri dev
```

## Building

```bash
# Build for production
cargo tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Tech Stack

- **Backend**: Tauri 2.0
- **WebView**: Loads Apple Music directly

## Troubleshooting

### White screen on Linux

Make sure you have WebKit2GTK installed:

```bash
# Arch Linux
sudo pacman -S webkit2gtk

# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-37
```

### "Cannot open page" error

This is normal - the app loads Apple Music's website directly in the native WebView, not through a local frontend.

## Disclaimer

This project is for personal use. Apple Music is a trademark of Apple Inc.
