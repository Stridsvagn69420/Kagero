export interface Config {
    git: GitConfig,
    repos: string[]
}
interface GitConfig {
    user: string,
    email: string,
    token: string
}