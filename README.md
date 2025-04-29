# Build Instructions

## Install rust toolchain

Install [rustup](https://rustup.rs/)

## Install dependencies

### Linux

Fedora and derivatives:
```bash
sudo dnf install gtk4-devel gcc libadwaita-devel poppler-glib-devel # untested
```

Debian and derivatives:
```bash
sudo apt install libgtk-4-dev build-essential libadwaita-1-dev libpoppler-glib-dev
```

Arch and derivatives:
```bash
sudo pacman -S gtk4 base-devel libadwaita poppler-glib # untested
```

### Mac

Install [homebrew](https://crates.io/crates/sourceview5)

MacOS:
```bash
brew install gtk4 libadwaita poppler-glib # untested
```

## Install xelatex

Debian and derivatives:
```bash
sudo apt install texlive-xetex # minimal installation (should be enough?)
```
```bash
sudo apt install texlive-full # complete latex installation
```
