package config

type Config struct {
	Server    ServerConfig    `yaml:"pgshub"`
	MySql     MySqlConfig     `yaml:"mysql"`
	Jwt       JwtConfig       `yaml:"jwt"`
	Container ContainerConfig `yaml:"container"`
}
