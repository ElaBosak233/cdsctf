package config

type ServerConfig struct {
	Host string `yaml:"host"`
	Port int    `yaml:"port"`
}
