package entity

import "time"

// Challenge is the challenge for Jeopardy-style CTF game.
type Challenge struct {
	ChallengeID   int64     `xorm:"'id' pk autoincr" json:"id"`                                         // The challenge's id. As primary key.
	Title         string    `xorm:"'title' nvarchar(32) notnull" json:"title"`                          // The challenge's title.
	Description   string    `xorm:"'description' text notnull" json:"description"`                      // The challenge's description.
	Category      string    `xorm:"'category' varchar(16) notnull" json:"category"`                     // The challenge's category.
	HasAttachment *bool     `xorm:"'has_attachment' bool notnull default(false)" json:"has_attachment"` // Whether the challenge has attachment.
	IsPracticable *bool     `xorm:"'is_practicable' bool notnull default(false)" json:"is_practicable"` // Whether the challenge is practicable. (Is the practice field visible.)
	IsDynamic     *bool     `xorm:"'is_dynamic' bool default(false)" json:"is_dynamic"`                 // Whether the challenge is based on dynamic container.
	ExposedPort   int       `xorm:"'exposed_port' int" json:"exposed_port,omitempty"`                   // The exposed port of the challenge's container image.
	Image         string    `xorm:"'image' text" json:"image,omitempty"`                                // The container image name of the challenge. (Such as "nginx:latest")
	Flag          string    `xorm:"'flag' varchar(128)" json:"flag,omitempty"`                          // The static flag of the challenge.
	FlagEnv       string    `xorm:"'flag_env' varchar(16)" json:"flag_env,omitempty"`                   // The environment variable of the challenge when the challenge is based on dynamic container.
	FlagFmt       string    `xorm:"'flag_fmt' varchar(64)" json:"flag_fmt,omitempty"`                   // The flag format of the challenge when the flag is dynamic.
	CpuLimit      float64   `xorm:"'cpu_limit' default(1)" json:"cpu_limit,omitempty"`                  // CPU limit. (Core)
	MemoryLimit   int64     `xorm:"'memory_limit' default(256)" json:"memory_limit,omitempty"`          // Memory limit. (MB)
	Duration      int64     `xorm:"'duration' default(1800)" json:"duration,omitempty"`                 // The duration of container maintenance in the initial state. (Seconds)
	Difficulty    int64     `xorm:"'difficulty' default(1)" json:"difficulty"`                          // The degree of difficulty. (From 1 to 5)
	PracticePts   int64     `xorm:"'practice_pts' default(200) notnull" json:"practice_pts"`            // The points will be given when the challenge is solved in practice field.
	CreatedAt     time.Time `xorm:"'created_at' created" json:"created_at"`                             // The challenge's creation time.
	UpdatedAt     time.Time `xorm:"'updated_at' updated" json:"updated_at"`                             // The challenge's last update time.
}

func (c *Challenge) TableName() string {
	return "challenge"
}
