package main

import (
	"Kagero/commands"
	"os"
)

func main() {
	switch os.Args[1] {
	case "install":
		commands.Install(os.Args[2])
	case "sync":
		commands.Sync()
	default:
		// Print help and exit with 1
	}
}
