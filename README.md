# Kagero
A cross-platform universal and easy to use package manager

# Installing
## Go SDK
Note: You can install Kagero with this method, but also update it.
1. Clone the repo, e.g. in `~/go/src`, with `git clone https://github.com/Stridsvagn69420/Kagero.git`
2. Go into the newly created folder with `cd Kagero`
3. Run `make`/`make install` or `go install src/kagero.go` to directly install Kagero via the Go SDK

## APT
Coming soon.

## Pacman
Coming soon.

## Compiling from source
1. Clone the repo, e.g. in `~/go/src`, with `git clone https://github.com/Stridsvagn69420/Kagero.git`
2. Go into the newly created folder with `cd Kagero`
3. Run `make windows` on Windows and `make linux` on Unix-like systems to compile it from source
4. Copy the built executable into a folder listed in your PATH variable, e.g. `cp build/kagero /usr/bin/kagero`

# Usage
Note: This is just some of the most common tasks. Please take a look at [the complete docs](/docs/README.md) for the entire documentation
## Installing packages
Coming soon.

## Updating packages
Coming soon.

## Removing packages
Coming soon.

## Managing repositories
Coming soon.

# FAQ
## Why is it called Kagero?
1. I'm obsessed with Fire Emblem and I thought [Kagero](https://fireemblem.fandom.com/wiki/Kagero) would fit.
2. It sounds a bit like Rust's package manager [Cargo](https://crates.io/).

## Should I still use my system package manager (pacman, apt, rpm, etc.)
**Yes, absolutely!** Your system package manager is far more advanced and still neccesary for updating the kernel/core elements. Currently Kagero is only for non-system software like games, applications that don't extend the core operating system (or have essential files in system folders), or fun projects.  
**It's not ready for being a system package manager yet!**
* Something that needs deep access might work, but would be complicated since every package is stored in its own folder, sometimes globally or locally.  
* I also doubt you'll get first party package support for something like the Go SDK, .NET SDK, Flutter/Dart SDK, Rust SDK - maybe with some projects on GitHub like [btop++](https://github.com/aristocratos/btop), [yt-dlp](https://github.com/yt-dlp/yt-dlp), [YT-Spammer-Purge](https://github.com/ThioJoe/YT-Spammer-Purge), etc.  
* The only realistic first-party support would be with projects I made and maybe with some of my friends' projects.