# Orochi Repository
[Orochi](https://github.com/Stridsvagn69420/Orochi) is the recommended tool to create and manage, optionally host repositories for the Kagero package manager.

# Layout
The root of your repository should look like this:
```
/var/lib/your-repo or kagero.yourrepo.com or www.your-repo.org/orochi
├── linux/
|   ├── runtime/
│   ├── aarch64/
│   └── x86_64/
│
├── windows/
|   ├── runtime/
│   ├── aarch64/
│   └── x86_64/
│
├── shurikens/
|   ├── package1.shuriken
│   ├── binary2.shuriken
│   ........
│   ├── binary69.shuriken
│   └── package-n.shuriken
│
├── other/
|   ├── minecraft/
│   ........
│   └── wallpaper/
│
└── orochi.katana
```
This is the recommended way for the layout, but settings can be tweaked in the [Katana file](Katana.md) if needed and only the folder actually used have to be created.  
Note: Currently only Windows and Linux systems on `aarch64` and `x86_64` are actively supported.

## orochi.katana
The entry point of your repo. It includes metadata about your repo, packages, and file structure. It can be named anything, but something related to Kagero or Orochi is recommended.  
See the [Katana specs](Katana.md) for more.

## shurikens/
While the other folders contain the package data itself, it's metadata and pointer is stored in [Shuriken files](Shuriken.md) which allows a package Shuriken to be made once, but pointed to mutliple platforms depending on what is supported.

## OS-TYPE/CPU-ARCH/
These folders contain the actual package data like pre-compiled executables or platform specific libaries, e.g. a Linux application or WinRT library for language XY. Packages that are special like `mcbedrock-resource` should be stored in a different folder like `other/minecraft/` for a clearer file tree.

### runtime
This special folder includes cross-arch/arch-independent executables like Python/Node.js/JVM projects and Shell scripts. These include package types like:
* JVM `jvm`
* .NET `dotnet`
* Python `python`
* Node.js `nodejs`