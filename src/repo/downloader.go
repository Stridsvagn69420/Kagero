package repo

import (
	"Kagero/handles"
	"errors"
	"io"
	"io/ioutil"
	"net/http"
	"os"

	"github.com/go-git/go-git/v5"
	"github.com/go-git/go-git/v5/plumbing"
)

func DownloadFile(filepath string, url string) error {
	// Create the file
	out, err := os.Create(filepath)
	if err != nil {
		return err
	}
	defer out.Close()

	// Get the data
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// Check server response
	if resp.StatusCode != http.StatusOK {
		return errors.New("The Server responded with a status code other than 200:\n" + resp.Status)
	}

	// Writer the body to file
	_, err = io.Copy(out, resp.Body)
	if err != nil {
		return err
	}

	return nil
}

func DownloadString(url string) string {
	resp, err := http.Get(url)
	handles.HttpErr(err)
	defer resp.Body.Close()

	// Check server response
	if resp.StatusCode != http.StatusOK {
		handles.HttpErr(errors.New("The Server responded with a status code other than 200:\n" + resp.Status))
	}

	// Writer the body to file
	body, err := ioutil.ReadAll(resp.Body)
	handles.HttpErr(err)

	// Return string
	return string(body)
}

func GitClone(giturl string, branch string, localdir string) {
	// Clone the given repository to the given directory with given branch
	_, err := git.PlainClone(localdir, false, &git.CloneOptions{
		ReferenceName: plumbing.NewBranchReferenceName(branch),
		URL:           giturl,
		Progress:      os.Stdout,
		SingleBranch:  true,
	})
	handles.GitErr(err)
}

func GitCloneDefault(giturl string, localdir string) {
	// Clone the given repository to the given directory with default branch
	_, err := git.PlainClone(localdir, false, &git.CloneOptions{
		URL:          giturl,
		Progress:     os.Stdout,
		SingleBranch: true,
	})
	handles.GitErr(err)
}
