package stdio

import (
	"log"
	"os"
)

var logger = log.New(os.Stdout, "", 0)
var errlogger = log.New(os.Stderr, "", 0)

const (
	// Reset
	reset = "\033[0m"

	// Font colors
	Black  = "\033[30m"
	Red    = "\033[31m"
	Green  = "\033[32m"
	Yellow = "\033[33m"
	Blue   = "\033[34m"
	Purple = "\033[35m"
	Cyan   = "\033[36m"
	White  = "\033[37m"

	// Background colors
	BBlack  = "\033[40m"
	BRed    = "\033[41m"
	BGreen  = "\033[42m"
	BYellow = "\033[43m"
	BBlue   = "\033[44m"
	BPurple = "\033[45m"
	BCyan   = "\033[46m"
	BWhite  = "\033[47m"
)

// Standard Printers
func PrintColor(color string, message string) {
	logger.Print(color + message + reset)
}

func PrintLineColor(color string, message string) {
	logger.Println(color + message + reset)
}

// Error Printers
func ErrorColor(color string, message string) {
	errlogger.Print(color + message + reset)
}

func ErrorLineColor(color string, message string) {
	errlogger.Println(color + message + reset)
}
