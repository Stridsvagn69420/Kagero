interface Pointer {
    hash: "\b[A-Fa-f0-9]{64}\b",
    platform: "windows" | "linux" | "any" | "other",
    arch: "aarch64" | "x86_64" | string,
    data: Data
}

interface Data {
    path: string | `git+${string}` | `http+${string}`,
    type: "exe" | "zip" | "tar+xz" | "tar+gzip"
}

export type Shuriken = Pointer[];