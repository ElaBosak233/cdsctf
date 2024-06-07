package files

import (
	"embed"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"os"
	"path"
)

var (
	//go:embed * statics/* templates/* i18n/*
	fs embed.FS
)

func F() embed.FS {
	return fs
}

func ReadStaticFile(filename string) (data []byte, err error) {
	if _, err = os.Stat(path.Join(utils.FilesPath, "statics", filename)); err == nil {
		data, err = os.ReadFile(path.Join(utils.FilesPath, "statics", filename))
	} else {
		data, err = F().ReadFile("statics/" + filename)
	}
	return data, err
}

func ReadTemplateFile(filename string) (data []byte, err error) {
	if _, err = os.Stat(path.Join(utils.FilesPath, "templates", filename)); err == nil {
		data, err = os.ReadFile(path.Join(utils.FilesPath, "templates", filename))
	} else {
		data, err = F().ReadFile("templates/" + filename)
	}
	return data, err
}
