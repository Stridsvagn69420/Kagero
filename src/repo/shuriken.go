package repo

import (
	"Kagero/handles"
	"encoding/json"
)

func Shuriken(data string) shuriken {
	var k shuriken
	err := json.Unmarshal([]byte(data), &k)
	handles.JsonErr(err)
	return k
}

type shuriken struct {
	Package  string   `json:"package"`
	Hash     string   `json:"hash"`
	Platform string   `json:"platform"`
	Arch     string   `json:"arch"`
	Data     dataConf `json:"data"`
}

type dataConf struct {
	Path string `json:"path"`
	Type string `json:"type"`
}
