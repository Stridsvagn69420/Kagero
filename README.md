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

<hr>

# The Kagero-Package-Manager Project
## History
The Kagero Package Manager originally started as `jitter` and was supposed to be written in Dart to use Flutter as a GUI later. This dropped later because I discovered Go and Dart didn't really fit anymore. I then decided to ditch Go - I didn't even really start with it - in favor of Rust.  
The main reason why I wanted to make "jitter" was because shipping a multi-file app wtihout a package manager (Windows doesn't have any and some Linux distros have a complicated way of setting up a repository) is pain. The "jit" stood for Just-in-Time, because I wanted to package [my Twitter downloader](https://github.com/Stridsvagn69420/Tweet-DL).

## Comparision
|                | [Kagero](https://github.com/Stridsvagn69420/Kagero)                               | [Orochi](https://github.com/Stridsvagn69420/Orochi)                                   | [Kaze](https://github.com/Stridsvagn69420/Kaze)                  | [libkagero](https://github.com/Stridsvagn69420/libkagero)              | [DDroid](https://github.com/Stridsvagn69420/DDroid)            |
| -------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------------------- | -------------------------------------------------------------- |
| Origin of Name | [Hoshidan Ninja](https://fireemblem.fandom.com/wiki/Kagero) and retainer of Ryoma | [Hoshidan Diviner](https://fireemblem.fandom.com/wiki/Orochi) who served Queen Mikoto | [Hoshidan Ninja](https://fireemblem.fandom.com/wiki/Kaze)         | (Kagero with a lib-prefix)                                             | D(ecentralized An)Droid                                        |
| Teck Stack     | Rust                                                                              | Rust, Actix-Web (Nginx)                                                               | C#, .NET, GtkSharp, GTK+, Glade                                  | C# Class-Library, Kotlin-Multiplatform, NPM Package, Dart Package      | Kotlin, Android-App                                            |
| Target         | Windows, Linux, FreeBSD, NetBSD, OpenBSD (macOS, Unix supported)                  | Linux, Unix, FreeBSD, OpenBSD, NetBSD (Windows, macOS supported)                      | Windows, Linux, FreeBSD, NetBSD, OpenBSD (macOS, Unix supported) | \<InsertYourProjectPlatformHere\>                                        | Android, Android x86, HarmonyOS, Windows 11, Bluestacks        |
| Role           | TTY/Command-Line Package Manager Client                                           | TTY/Command-Line Repository Manager and Server                                        | GUI (optionally command-line) Package Manager                    | Official libraries to integrate the Kagero-Orochi system into your app | Mobile Android App Store similar to the Kagero Package Manager |

<hr>

# FAQ
## Why is it called Kagero?
1. I'm obsessed with Fire Emblem and I thought [Kagero](https://fireemblem.fandom.com/wiki/Kagero) would fit.
2. It sounds a bit like Rust's package manager, [Cargo](https://crates.io/).

## Should I still use my system package manager (pacman, apt, rpm, etc.)
**Yes, absolutely!**  
Your system package manager is far more advanced and still neccesary for updating the kernel/core elements. Currently Kagero is only for non-system software like games, applications that don't extend the core operating system (or have essential files in system folders), or fun projects.

## Why have 2 Package Managers with one having an extra GUI?
There are many reason why I dediced to split it up:

### Kagero is the main and Kaze gets pulled behind
Basically, Kagero as a library and package manager gets all the features first. It's supposed to be a simple and easy package manager, so adding a GUI that 99% of users won't use would just bloat the binary and make compiling more difficult (at least on Windows). Plus, I expect this to be full of changes, so having them developed in parralel would be a hassle, and Kagero thus also functions as some kind of "test-bed". Kaze, written in C# and GtkSharp being very easy to use, functions I guess as the Debian of Kagero. Though, when there's a new revision of Kagero, libkagero/Kaze will be bumped as soon as possible. APIs are supposed to be similar anyway.

### gtk-rs and Windows
The problem I encountered with Rust and a cross-platform GUI is GTK+. It's way easier to include Glade layouts in .NET because of the `.csproj` configurations, but GtkSharp (the not-Mono version), doesn't need any extra software installed, whereas Rust needs either MSVC or MSYS2 installed and this can be a hassle for a beginner-dev to try and contribute to a cool idea.

## What is libkagero and why isn't the Rust version part of it?
Like I (tried to) describe, Kagero aka. this repository and Orochi, the official server and repository manager, both written in Rust, are everything needed. Kagero serves as a binary but also crate for Orochi. Despite this, you cna use it for your own Rust projects as well.  
`libkagero` is a repository that features official supported libraries for various languages, e.g. Kotlin, C#/.NET, Dart, etc. They're not used in any core element, but can be used to implement the entire Orochi Repository System with your game, own package manager, tool, application, etc.
