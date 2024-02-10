package config

type Config struct {
	Global    Global    `yaml:"global" json:"global" mapstructure:"global"`
	Server    Server    `yaml:"server" json:"server" mapstructure:"server"`
	Email     Email     `yaml:"email" json:"email" mapstructure:"email"`
	Db        Db        `yaml:"db" json:"db" mapstructure:"db"`
	Jwt       Jwt       `yaml:"jwt" json:"jwt" mapstructure:"jwt"`
	Container Container `yaml:"container" json:"container" mapstructure:"container"`
}

type Global struct {
	Platform  GlobalPlatform  `yaml:"platform" json:"platform" mapstructure:"platform"`
	Container GlobalContainer `yaml:"container" json:"container" mapstructure:"container"`
	User      GlobalUser      `yaml:"user" json:"user" mapstructure:"user"`
}

type GlobalPlatform struct {
	Title       string `yaml:"title" json:"title" mapstructure:"title"`
	Description string `yaml:"description" json:"description" mapstructure:"description"`
}

type GlobalContainer struct {
	ParallelLimit int `yaml:"parallel_limit" json:"parallel_limit" mapstructure:"parallel_limit"`
	RequestLimit  int `yaml:"request_limit" json:"request_limit" mapstructure:"request_limit"`
}

type GlobalUser struct {
	AllowRegistration bool `yaml:"allow_registration" json:"allow_registration" mapstructure:"allow_registration"`
}

type Server struct {
	Host string     `yaml:"host" json:"host" mapstructure:"host"`
	Port int        `yaml:"port" json:"port" mapstructure:"port"`
	CORS ServerCORS `yaml:"cors" json:"cors" mapstructure:"cors"`
}

type ServerCORS struct {
	AllowOrigins []string `yaml:"allow_origins" json:"allow_origins" mapstructure:"allow_origins"`
	AllowMethods []string `yaml:"allow_methods" json:"allow_methods" mapstructure:"allow_methods"`
}

type Email struct {
	Address  string    `yaml:"address" json:"address" mapstructure:"address"`
	Password string    `yaml:"password" json:"password" mapstructure:"password"`
	SMTP     EmailSMTP `yaml:"smtp" json:"smtp" mapstructure:"smtp"`
}

type EmailSMTP struct {
	Host string `yaml:"host" json:"host" mapstructure:"host"`
	Port int    `yaml:"port" json:"port" mapstructure:"port"`
}

type Db struct {
	Provider string     `yaml:"provider" json:"provider" mapstructure:"provider"`
	Postgres DbPostgres `yaml:"postgres" json:"postgres" mapstructure:"postgres"`
	Sqlite3  DbSqlite3  `yaml:"sqlite3" json:"sqlite3" mapstructure:"sqlite3"`
}

type DbPostgres struct {
	Host     string `yaml:"host" json:"host" mapstructure:"host"`
	Port     int    `yaml:"port" json:"port" mapstructure:"port"`
	Username string `yaml:"username" json:"username" mapstructure:"username"`
	Password string `yaml:"password" json:"password" mapstructure:"password"`
	Dbname   string `yaml:"dbname" json:"dbname" mapstructure:"dbname"`
	Sslmode  string `yaml:"sslmode" json:"sslmode" mapstructure:"sslmode"`
}

type DbSqlite3 struct {
	Filename string `yaml:"filename" json:"filename" mapstructure:"filename"`
}

type Jwt struct {
	SecretKey  string `yaml:"secret_key" json:"secret_key" mapstructure:"secret_key"`
	Expiration int    `yaml:"expiration" json:"expiration" mapstructure:"expiration"`
}

type Container struct {
	Provider string          `yaml:"provider" json:"provider" mapstructure:"provider"`
	Docker   ContainerDocker `yaml:"docker" json:"docker" mapstructure:"docker"`
}

type ContainerDocker struct {
	URI         string `yaml:"uri" json:"uri" mapstructure:"uri"`
	PublicEntry string `yaml:"public_entry" json:"public_entry" mapstructure:"public_entry"`
}
