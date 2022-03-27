package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"runtime"

	"github.com/BurntSushi/toml"
)

type Config struct {
	Urls []string
}

func main() {
	config, err := getConfig()
	if err != nil {
		log.Fatal(err)
	}

	for index, url := range config.Urls {
		fmt.Printf("%3d: %v\n", index+1, url)
		openBrowser(url)
	}
}

func getConfig() (conf *Config, err error) {
	bytes, err := os.ReadFile("D:\\ProgramData\\bin\\jiayou.toml")
	if err != nil {
		return nil, err
	}

	var config Config
	_, err = toml.Decode(string(bytes), &config)
	if err != nil {
		return nil, err
	}

	return &config, nil
}

func openBrowser(url string) {
	var err error

	switch runtime.GOOS {
	case "linux":
		err = exec.Command("xdg-open", url).Start()
	case "windows":
		err = exec.Command("rundll32", "url.dll,FileProtocolHandler", url).Start()
	case "darwin":
		err = exec.Command("open", url).Start()
	default:
		err = fmt.Errorf("unsupported platform")
	}

	if err != nil {
		log.Fatal(err)
	}
}
