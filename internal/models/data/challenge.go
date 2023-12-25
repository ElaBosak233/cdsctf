package data

// Challenge 题目
type Challenge struct {
	ChallengeId   string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	Title         string `xorm:"'title' varchar(50) notnull" json:"title"`
	Description   string `xorm:"'description' text notnull" json:"description"`
	Category      string `xorm:"'category' varchar(16) notnull" json:"category"`
	HasAttachment bool   `xorm:"'has_attachment' bool notnull" json:"has_attachment"`
	IsPracticable bool   `xorm:"'is_practicable' bool notnull" json:"is_practicable"`
	IsDynamic     bool   `xorm:"'is_dynamic' bool" json:"is_dynamic"`
	ExposedPort   int    `xorm:"'exposed_port' int" json:"exposed_port"`
	Image         string `xorm:"'image' text" json:"image"`
	Flag          string `xorm:"'flag' varchar(255)" json:"flag"`
	FlagEnv       string `xorm:"'flag_env' varchar(16)" json:"flag_env"`
	FlagPrefix    string `xorm:"'flag_prefix' varchar(16)" json:"flag_prefix"`
	MemoryLimit   int64  `xorm:"'memory_limit' int" json:"memory_limit"`
	Duration      int    `xorm:"'duration' int" json:"duration"`
	Difficulty    int    `xorm:"'difficulty' int" json:"difficulty"`
	CreatedAt     int64  `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt     int64  `xorm:"updated 'updated_at'" json:"updated_at"`
}
