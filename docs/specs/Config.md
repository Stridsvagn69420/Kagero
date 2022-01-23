# Config files
The docs for Kagero's config file. It states the plural in the title, because there's a system-wide one (optional) and the local one (required) that overrides values in the global one.

# Structure
```json
{
    "git": {
        "name": "EpicDev",
        "email": "gitmail@myprovider.com",
        "token": "HahaYesIHaveSecurePassword100"
    },
    "repos": [
        "https://EpicDev.github.io/Repo/Kagero/orochi.katana",
        "https://orochi.myhost.com/index.katana"
    ]
}
```

## git
Username and Password for cloning Git repositories. You only need authentication when Shurikens in a repository point to a private Git-Repo.

## repos
An array of URLs that point to a repos Katana files.

# Locations
Your local config resides in `~/.kagero/config.json`, while the global one is located in `/etc/kagero/config.json`.  
Windows doesn't have any global options or paths, so it's config is in `~/AppData/Local/Kagero/config.json`