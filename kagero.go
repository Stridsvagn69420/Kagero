package main

import (
	"os"

	"github.com/Stridsvagn69420/Kagero/commands"
)

func main() {
	switch os.Args[1] {
	case "install":
		commands.Install(os.Args[2])
	case "update":
		commands.Update()
	default:
		// Print help and exit with 1
	}
}
