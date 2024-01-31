package assets

import (
	"github.com/elabosak233/pgshub/assets"
	"os"
)

func ReadStaticFile(filename string) (data []byte, err error) {
	if _, err = os.Stat("./assets/statics/" + filename); err == nil {
		data, err = os.ReadFile("./assets/statics/" + filename)
	} else {
		data, err = embed.FS.ReadFile("statics/" + filename)
	}
	return data, err
}

func ReadTemplateFile(filename string) (data []byte, err error) {
	if _, err = os.Stat("./assets/templates/" + filename); err == nil {
		data, err = os.ReadFile("./assets/templates/" + filename)
	} else {
		data, err = embed.FS.ReadFile("templates/" + filename)
	}
	return data, err
}
