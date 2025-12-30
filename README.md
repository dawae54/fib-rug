# fib-rug
Basic fibonacci numbers calculator using the rug crate.
# Installation
1. Make sure that `clang` is installed:
### Debian/Ubuntu
```sh
sudo apt install clang
```
### Fedora/RedHat
```sh
sudo dnf install clang
```
### Arch Linux
```sh
sudo pacman -S clang
```

2. Export `clang` as c compiler:
```sh
export CC=clang
```
3. After exporting `clang` , install the package using cargo:
```sh
cargo install fig-rug
```
# Usage
```
fib <number>
```

![gif](fib.gif)
