package handles

import (
	"Kagero/stdio"
)

func JsonErr(err error) {
	if err != nil {
		stdio.ErrorLineColor(stdio.Red, "An error occured while parsing the JSON file:")
		stdio.ErrorLineColor(stdio.Red, err.Error())
	}
}

func HttpErr(err error) {
	if err != nil {
		stdio.ErrorLineColor(stdio.Red, "An error occured while downloading the file:")
		stdio.ErrorLineColor(stdio.Red, err.Error())
	}
}

func GitErr(err error) {
	if err != nil {
		stdio.ErrorLineColor(stdio.Red, "An error occured while cloning the repository:")
		stdio.ErrorLineColor(stdio.Red, err.Error())
	}
}
