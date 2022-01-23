# Shuriken files
Pointer-/Metadata files for Kagero packages

# Structure
Please look at the [TypeScript data for Shuriken](Shuriken.d.ts) files to see a clearer structure and all possible values.
```json
[{
    "package": "org.dx.11",
    "hash": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "platform": "windows",
    "arch": "x86_64",
    "data": {
        "path": "my-epic-app.exe",
        "type": "standalone-exe"
    }
},
{
    "package": "com.stuff.raspberrypi",
    "hash": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "platform": "linux",
    "arch": "aarch64",
    "data": {
        "path": "my-epic-pi.zip",
        "type": "zip"
    }
}]
```
Note: `platform`, `arch`, and `path` are used for Kagero's URL Builder: `${ROOT}/${platform}/${arch}/${path}`. The package manager will first look through the Array and check if the package is even available for the current system, and then check `path`. If it's just a standard file name, it will make a request like stated before, but if `path` has

## package
A package ID that will be used to name the folder your package resides in.

## hash
The SHA-256 hash value of the data that will be downloaded. This will make sure everything was downloaded properly. You can ignore this for Git-URLs.

## platform
The OS type the package was built for. `linux` primarily means Linux, but it should be compatible with FreeBSD, OpenBSD, and every other Unix-like systems

## arch
The CPU architecture the package was built for. When `platform` is set to `other`, this will be program type the package belongs to, and when set to `any`, this entry will be skipped.

## data
### path
The filename of the executable/archive inside `${ROOT}/${platform}/${arch}` to be downloaded, unless it's one of these special cases:
* `git+<HTTP(S)-Git-URL>`: Kagero will clone the Git repository specified. A [Dagger file](Dagger.md) in the repos root is needed for Kagero if `type` is set to `other` so that the package can be used correctly.
* `http+<HTTP(S)-URL>`: Kagero will get the data from the specified URL. Useful if you already have it somewhere else and don't want duplicates.

### type
The type of data downloaded:
* `exe`: Just a stanalone single-file executable
* `aur`: (Arch Linux only) A package of the [Arch User Repositor](https://aur.archlinux.org/) or like one.
* `zip`: A .zip file with its content having a file tree like described [in the Package section](#packages)
* `tar+gzip`: Like above but as a .tar.gz file.
* `tar+xz`: Like above but as a .tar.xz file.

# Packages
Packages can look like whatever they want to. They just need 2 files: `deps.txt` and `pathbin.txt`

## deps.txt
A list of commands required to be installed/inside PATH:
```
git
gpg
node
npm
```

## pathbin.txt
A list of files to be symlinked into Kagero's PATH folder, similar to a `.gitignore`:
```
bin/master
scripts/update.sh
windows/run.bat
main.ps1
```