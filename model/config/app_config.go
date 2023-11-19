package config

type Config struct {
	Server    ServerConfig    `yaml:"server"`
	Database  DatabaseConfig  `yaml:"database"`
	Jwt       JwtConfig       `yaml:"jwt"`
	Container ContainerConfig `yaml:"container"`
}
