package config

type Ports struct {
	From int `yaml:"from"`
	To   int `yaml:"to"`
}

type ContainerConfig struct {
	Host  string `yaml:"host"`
	Ports Ports  `yaml:"ports"`
}
