# Kagero
A cross-platform universal and easy to use package manager.  
This crate is a binary as well as a library and features Kagero-specific filesystem utils and Orochi-specific data types and utils, but also a general-purpose Printer utility to read from and write to the command-line, with color too.

# Installing
## Cargo (Rust SDK)
Run `cargo install kagero` and you're done! If you haven't already, add `$HOME/.cargo/bin` to your PATH.

## Pacman (Arch Linux, Artix Linux, Manjaro)
Coming soon.

## XBPS (Void Linux)
Coming soon.

## APT (Debian, Ubuntu, Linux Mint)
Coming soon.

## DNF (Fedora, RHEL)

## Compiling from Source (Rust SDK)
1. Clone the repo with `git clone https://github.com/Stridsvagn69420/Kagero.git`
2. Either directly install it: `cargo install --path Kagero` (Make sure `$HOME/.cargo/bin` is in your PATH)
3. OR Go into the newly created folder with `cd Kagero`
4. And run `cargo build --release` and copy the binary located at `./target/release/kagero[.exe]`

# Usage
WIP.

# FAQ
## Why is it called Kagero?
1. I'm obsessed with Fire Emblem and I thought [Kagero](https://fireemblem.fandom.com/wiki/Kagero) would fit.
2. It sounds a bit like Rust's package manager, [Cargo](https://crates.io/).

## Should I still use my system package manager (pacman, apt, rpm, etc.)
**Yes, absolutely!**  
Your system package manager is far more advanced and still neccesary for updating the kernel/core elements. Currently Kagero is only for non-system software like games, applications that don't extend the core operating system (or have essential files in system folders), or fun projects.
