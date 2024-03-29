package main

import (
	"errors"
	"flag"
	"fmt"
	"io/fs"
	"log"
	"os"
	"path"
	"path/filepath"
	"strings"

	"github.com/fatih/color"
)

const LINE_WORDS = 6

func main() {
	listCmd := flag.Bool("list", false, "list tips")
	flag.Parse()

	tipDir := os.Getenv("TIP_DATA_ROOT")
	if tipDir == "" {
		tipDir = path.Join(os.Getenv("HOME"), "bin", "data", "tip")
	}

	fmt.Println("TIP_GO-v0.1-20220329.1")
	fmt.Printf("tipDir: %v\n", tipDir)

	args := flag.Args()
	if *listCmd || len(args) == 0 {
		doListTips(tipDir)
	} else {
		doShowTip(tipDir, args[0])
	}
}

func doListTips(tipDir string) {
	color.Set(color.FgGreen)

	var i = 0
	filepath.Walk(tipDir, func(path string, _info fs.FileInfo, err error) error {
		fmt.Printf("%-17s", filenameWithoutExtension(filepath.Base(path)))
		i += 1
		if i%LINE_WORDS == 0 {
			fmt.Println()
		}
		return err
	})

	color.Unset()
}

func doShowTip(tipDir string, t string) {
	fileName := fmt.Sprintf("%s.md", path.Base(t))
	fullPath := path.Join(tipDir, fileName)
	if _, err := os.Stat(fullPath); errors.Is(err, os.ErrNotExist) {
		fmt.Printf("%s not exists\n", color.RedString(fullPath))
	} else {
		content, err := os.ReadFile(fullPath)
		if err != nil {
			log.Fatalln(err)
		} else {
			fmt.Printf("%s\n", color.BlueString(string(content)))
		}
	}
}

func filenameWithoutExtension(s string) string {
	return strings.TrimSuffix(s, path.Ext(s))
}
