package config

type ConfigJSON struct {
	Git struct {
		Name  string `json:"name"`
		Email string `json:"email"`
		Token string `json:"token"`
	} `json:"git"`
	Repos []string `json:"repos"`
}
