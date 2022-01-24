package config

type gitConfig struct {
	Name  string `json:"name"`
	Email string `json:"email"`
	Token string `json:"token"`
}

type ConfigJSON struct {
	Git   gitConfig `json:"git"`
	Repos []string  `json:"repos"`
}
