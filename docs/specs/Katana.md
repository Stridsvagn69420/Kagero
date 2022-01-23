# Katana files
Pointer-/Metadata files for [Orochi-like](https://github.com/Stridsvagn69420/Orochi) repositories

# Structure
Please look at the [TypeScript data for Katana](Katana.d.ts) files to see a clearer structure and all possible values.
```json
{
    "uuid": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
    "website": "https://www.yourwebsite.com/",
    "root": "https://orochi.yourwebsite.com/",
    "name": "My Kagero Repository",
    "maintainers": [{
        "name": "Your Name",
        "email": "yourname@yourdomain.com",
        "profile": "https://github.com/YourName"
    }],
    "shurikens": [
        "my-package",
        "anotherpackage",
        "mooooooore"
    ]
}
```
## uuid
A simple UUID v4 as a unique identifier.

## website
A website for your repository or just about you, set to `null` if you don't have one/want to set it.

## root
The root of your repository. This is required for Kagero's request builder.

## name
What it says. Only needed for a rich representation.

## shurikens
A list of packages you provide. Kagero uses this key as an index for your packages. The "Get Shuriken" URL will be constructed after `${root}/shurikens/${shurikens[i]}.shuriken` and availability will be checked with a request to this URL.