package repo

import (
	"Kagero/handles"
	"encoding/json"
)

func Katana(data string) katana {
	var k katana
	err := json.Unmarshal([]byte(data), &k)
	handles.JsonErr(err)
	return k
}

type katana struct {
	Uuid        string       `json:"uuid"`
	Website     string       `json:"website"`
	Root        string       `json:"root"`
	Name        string       `json:"name"`
	Maintainers []maintainer `json:"maintainers"`
	Shurikens   []shuriken   `json:"shurikens"`
}

type maintainer struct {
	Name    string `json:"name"`
	Email   string `json:"email"`
	Profile string `json:"profile"`
}
