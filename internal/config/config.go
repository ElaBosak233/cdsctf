package config

import (
	"github.com/elabosak233/pgshub/embed"
	"github.com/spf13/viper"
	"go.uber.org/zap"
	"io"
	"io/fs"
	"os"
	"reflect"
)

var (
	v1     *viper.Viper
	appCfg Config
)

func Cfg() *Config {
	return &appCfg
}

func InitConfig() {
	v1 = viper.New()
	configFile := "config.json"
	v1.SetConfigType("json")
	v1.SetConfigFile(configFile)
	if _, err := os.Stat(configFile); err != nil {
		zap.L().Warn("No configuration file found, default configuration file will be created.")

		// Read default configuration from embed
		defaultConfig, _err := embed.FS.Open("default.json")
		if _err != nil {
			zap.L().Error("Unable to read default configuration file.")
			return
		}
		defer func(defaultConfig fs.File) {
			_ = defaultConfig.Close()
		}(defaultConfig)

		// Create config file in current directory
		dstConfig, _err := os.Create(configFile)
		defer func(dstConfig *os.File) {
			_ = dstConfig.Close()
		}(dstConfig)

		if _, _err = io.Copy(dstConfig, defaultConfig); _err != nil {
			zap.L().Error("Unable to create default configuration file.")
			panic(err)
		}
		zap.L().Info("The default configuration file has been generated.")
	}

	if err := v1.ReadInConfig(); err != nil {
		zap.L().Error("Unable to read configuration file.")
		return
	}

	if err := v1.Unmarshal(&appCfg); err != nil {
		zap.L().Error("Unable to parse configuration file to structure.")
	}
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
