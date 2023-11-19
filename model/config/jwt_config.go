package config

type JwtConfig struct {
	SecretKey      string `yaml:"secret_key"`
	ExpirationTime int64  `yaml:"expiration_time"`
}
