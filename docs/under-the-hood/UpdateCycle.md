# Update Cycle
How Kagero updates packages and package lists

# Package lists
Coming soon.

# Packages
## Git-Packages
Every time the package lists are synced, Kagero will check if the `hash` has changed. If it has, it first looks if commits were reverted. If it's not the case (most of the time), it will pull from the set origin.

## Other packages
Similar to [Git-Packages](#git-packages), it will look for a difference in the `hash`. The difference is that the hash here means the executable's/the decompressed archive's hash value.