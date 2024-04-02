package config

import (
	"crypto/ed25519"
	"encoding/base64"
	"github.com/spf13/viper"
	"go.uber.org/zap"
	"os"
	"path"
	"reflect"
)

var (
	v3     *viper.Viper
	sigCfg SignatureCfg
)

type SignatureCfg struct {
	PublicKey  string `yaml:"pub" json:"pub" mapstructure:"pub"`
	PrivateKey string `yaml:"pem" json:"pem " mapstructure:"pem"`
}

func SigCfg() *SignatureCfg {
	return &sigCfg
}

func InitSignatureCfg() {
	v3 = viper.New()
	configFile := path.Join("configs/signature.json")
	v3.SetConfigType("json")
	v3.SetConfigFile(configFile)

	if _, err := os.Stat(configFile); err != nil {
		publicKey, privateKey, _ := ed25519.GenerateKey(nil)
		sigCfg.PrivateKey = base64.StdEncoding.EncodeToString(privateKey)
		sigCfg.PublicKey = base64.StdEncoding.EncodeToString(publicKey)
		_ = sigCfg.Save()
	}
	if err := v3.ReadInConfig(); err != nil {
		zap.L().Error("Unable to read configuration file.")
		return
	}

	if err := v3.Unmarshal(&sigCfg); err != nil {
		zap.L().Error("Unable to parse configuration file to structure.")
	}
}

func (s *SignatureCfg) Save() (err error) {
	val := reflect.ValueOf(sigCfg)
	typeOfCfg := val.Type()

	for i := 0; i < val.NumField(); i++ {
		field := val.Field(i)
		v3.Set(typeOfCfg.Field(i).Tag.Get("mapstructure"), field.Interface())
	}
	err = v3.WriteConfig()
	return err
}
