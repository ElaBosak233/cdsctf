package config

import (
	"reflect"
)

func InitConfig() {
	NewApplicationCfg()
	NewPlatformCfg()
}

// SaveConfig is used to save(or sync) the configuration to the file
func SaveConfig() (err error) {
	val := reflect.ValueOf(appCfg)
	typeOfCfg := val.Type()

	for i := 0; i < val.NumField(); i++ {
		field := val.Field(i)
		v1.Set(typeOfCfg.Field(i).Tag.Get("mapstructure"), field.Interface())
	}
	err = v1.WriteConfig()
	return err
}
