# Installation
How packages are installed

# Archives (zip, tar.gz, tbz)
## Downloading
The archive's contents will be extracted into a folder named after the Shuriken's `package`-ID. The folder will be located in [the global or local package directory](../specs/Config.md#Locations), depending on if you set the global flag or not.

## Symlinking
Every file listed in [pathbin.txt](../specs/Shuriken.md#pathbintxt) will bes ymlinked to the global/local Kagero-Bin folder.

# Git-Repositories
## Downloading
The folder these reside in is the same like Archives, but their content is generated via cloning from Git rather than extracting an archive.

## Symlinking
Every file listed in [pathbin.txt](../specs/Shuriken.md#pathbintxt) will bes ymlinked to the global/local Kagero-Bin folder.

# Standalone executable
## Downloading
The folder these reside in is the same like Archives, but only the executable will be inside the folder.

## Symlinking
The executable will be symlinked to the global or local Kagero-Bin folder.