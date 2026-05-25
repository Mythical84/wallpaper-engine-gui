# Linux Wallpaperengine Gui

This app hopes to be a fully featured linux implementation of the wallpaper engine frontend. It uses [linux-wallpaperengine](https://github.com/Almamu/linux-wallpaperengine). This app is currently heavily under development, and many wallpaper engine features are not yet supported. Check the roadmap to see what has been implemented and what's planned to be implemented

---

### Table of Contents
- [Installation](#installation)
- [Building](#building)
- [Roadmap](#roadmap)
- [Contributing & Feedback](#contributing--feedback)

---

## Installation

### 1. Wallpaper Engine
You need to both own and install wallpaper engine on steam in order for [linux-wallpaperengine](https://github.com/Almamu/linux-wallpaperengine) to run your wallpapers.

### 2. linux-wallpaperengine
Go to the official github repository of linux-wallpaperengine, and find the installation instructions for your specific distro in the readme: https://github.com/Almamu/linux-wallpaperengine

### 3. Install Wallpaper Engine Gui
Grab the latest package from [GitHub Releases](https://github/Mythical83/wallpaper-engine-gui/releases) for your distro, or follow the instructions below to build from scratch

#### Debian based distributions

```bash
sudo apt install ./wallpaper-engine-gui_<version>_amd64.deb
```

#### Fedora / RHEL based distributions
```bash
sudo dnf install ./wallpaper-engine-gui_<version>_amd64.rpm
```

#### Appimage
```bash
./wallpaper-engine-gui_<version>_amd64.appimage
```

#### Other/Portable
```bash
./wallpaper-engine-gui
```

## Building
#### 1. Clone the repo:
```bash
git clone https://github.com/Mythical84/wallpaper-engine-gui
cd wallpaper-engine-gui
```

#### 2. Install required packages
```bash
npm/bun/pnpm/yarn install
```

#### 3. Build the package
If you want access to the appimage build, you must add NO_STRIP=true before the following command, due to a bug with the appimage build system used by tauri.

```bash
npm/bun/pnpm/yarn run tauri build
```

Once the build is finished, you'll be able to find the binary in the target/release folder, while the deb, rpm, and appimage will be in the target/release/bundle folder

## Roadmap
- [x] Set wallpaper engine wallpapers as backgrounds from the gui
- [ ] Automatic detection of wallpaper engine install directory
- [ ] Debug logs for wallpapers
- [ ] Ability to change base linux-wallpaperengine properties
- [ ] Ability to change wallpaper specific properties
- [ ] Workshop support
- [ ] UI overhaul

## Contributing & Feedback
To contribute, open a pull request, for feedback, open an issue.