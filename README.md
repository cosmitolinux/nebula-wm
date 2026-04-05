# N3bu1a shell

**N3bu1a** is a customized, high-performance Wayland shell environment designed for minimal aesthetics, fluid motion and a unique "Floating Island" workflow based off end4, while being low on resources.

## Dependencies

You need these installed before building:

- Rust (via [rustup](https://rustup.rs))
- Waybar
- Grim
- Slurp
- wl-copy (wl-clipboard)
- notify-send (libnotify)
- SWWW (or your wallpaper engine of choice)

On Arch/CachyOS:
```sh
sudo pacman -S waybar grim slurp wl-clipboard libnotify
yay -S swww  # or paru
```

## Building
```sh
git clone https://github.com/cosmitolinux/nebula-wm.git
cd nebula-wm
cargo build --release
```

## Install
```sh
sudo install -m 755 target/release/n3bu1a /usr/local/bin/
sudo ln -sf /usr/local/bin/n3bu1a /usr/local/bin/n3bu1a-ctl
```

Both `n3bu1a` and `n3bu1a-ctl` point to the same binary. no args = shell init, with a subcommand = controller.

## Usage

run with no arguments to start the shell:
```sh
n3bu1a
```

Or use `n3bu1a-ctl` for quick actions:
```sh
n3bu1a-ctl screenshot-area   # drag a box, saves + copies to clipboard
n3bu1a-ctl screenshot-full   # captures the whole screen
n3bu1a-ctl reload            # restart waybar if its acting up
```

Screenshots go to `~/Screenshots`.

## Why N3bu1a?

N3bu1a isnt just a config, its a rebuilt desktop experience. it takes the raw power of wayland and wraps it in a polished, consistent interface that feels premium out of the box.

- **floating island ui:** centered, pill-style bar that stays out of the way
- **custom logic:** built-in controls for system tasks and screenshots
- **1080p optimized:** every element scaled for full HD
- **motion design:** custom bezier curves for transitions that feel like glass

## Quality of Life additions

- **Spicetify** - comes with starry night theme
- **Vesktop Custom theme** - vencord but on linux
- **Smooth animations and terminal commands** - for the good looks
- **Chaotic-AUR** - automatic building aur packages
- **Oh My Fish** - equal to zsh and looks good
- **Hyprlock** - lock screen (custom one coming eventually)

This is a new project so dont expect the best

## Gallery

Screenshots coming soon. check the releases tab for the latest builds.

## Special thanks

- **Hyprland** - for the inspiration and foundation
- **The community** - for the endless customization ideas
- **End4** - for the google theming and illogical-impulse theming
- **Quickshell** - for the smooth animations out the box

*Developed by Cosmic and Lyon*
