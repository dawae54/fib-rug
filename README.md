# fib-rug
Basic fibonacci numbers calculator using the rug crate.
# Installation
Make sure that `clang` is installed:
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

Export `clang` as c compiler:
```bash
export CC=clang
```
After exporting `clang` , install the package using cargo:
```sh
cargo install fig-rug
```
# Usage
```
fib <number>
```
