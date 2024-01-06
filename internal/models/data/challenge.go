package data

// Challenge 题目
type Challenge struct {
	ChallengeId   string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	Title         string `xorm:"'title' varchar(50) notnull" json:"title"`                           // 题目标题
	Description   string `xorm:"'description' text notnull" json:"description"`                      // 题目描述
	Category      string `xorm:"'category' varchar(16) notnull" json:"category"`                     // 题目分类
	HasAttachment bool   `xorm:"'has_attachment' bool notnull default(false)" json:"has_attachment"` // 是否有附件
	IsPracticable bool   `xorm:"'is_practicable' bool notnull default(false)" json:"is_practicable"` // 是否为练习题
	IsDynamic     bool   `xorm:"'is_dynamic' bool default(false)" json:"is_dynamic"`                 // 是否为动态题目
	ExposedPort   int    `xorm:"'exposed_port' int" json:"exposed_port,omitempty"`                   // 容器暴露端口
	Image         string `xorm:"'image' text" json:"image,omitempty"`                                // 题目镜像
	Flag          string `xorm:"'flag' varchar(255)" json:"flag,omitempty"`                          // Flag 字符串（静态）
	FlagEnv       string `xorm:"'flag_env' varchar(16)" json:"flag_env,omitempty"`                   // Flag 环境变量（动态）
	FlagFmt       string `xorm:"'flag_fmt' varchar(64)" json:"flag_fmt,omitempty"`                   // Flag 格式（动态）
	MemoryLimit   int64  `xorm:"'memory_limit' int default(256)" json:"memory_limit,omitempty"`      // 内存限制（MB）
	Duration      int64  `xorm:"'duration' int default(1800)" json:"duration,omitempty"`             // 维持时间（秒）
	Difficulty    int64  `xorm:"'difficulty' int default(1)" json:"difficulty"`                      // 难度系数（1~5）
	PracticePts   int64  `xorm:"'practice_pts' int default(200) notnull" json:"practice_pts"`        // 练习得分
	CreatedAt     int64  `xorm:"'created_at' created" json:"created_at"`                             // 创建时间
	UpdatedAt     int64  `xorm:"'updated_at' updated" json:"updated_at"`                             // 更新时间
}
