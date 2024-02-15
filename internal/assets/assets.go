package assets

import (
	"fmt"
	"github.com/elabosak233/pgshub/embed"
	"github.com/elabosak233/pgshub/internal/config"
	"os"
)

func InitAssets() {
	if _, err := os.Stat(config.AppCfg().Gin.Paths.Assets); err != nil {
		err = os.Mkdir(config.AppCfg().Gin.Paths.Assets, os.ModePerm)
	}
}

func ReadStaticFile(filename string) (data []byte, err error) {
	if _, err = os.Stat(fmt.Sprintf("%s/statics/%s", config.AppCfg().Gin.Paths.Assets, filename)); err == nil {
		data, err = os.ReadFile(fmt.Sprintf("%s/statics/%s", config.AppCfg().Gin.Paths.Assets, filename))
	} else {
		data, err = embed.FS.ReadFile("statics/" + filename)
	}
	return data, err
}

func ReadTemplateFile(filename string) (data []byte, err error) {
	if _, err = os.Stat(fmt.Sprintf("%s/templates/%s", config.AppCfg().Gin.Paths.Assets, filename)); err == nil {
		data, err = os.ReadFile(fmt.Sprintf("%s/templates/%s", config.AppCfg().Gin.Paths.Assets, filename))
	} else {
		data, err = embed.FS.ReadFile("templates/" + filename)
	}
	return data, err
}
