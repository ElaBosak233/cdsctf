package data

// Challenge 题目
type Challenge struct {
	ChallengeId   string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	Title         string `xorm:"'title' varchar(50) notnull" json:"title"`                       // 题目标题
	Description   string `xorm:"'description' text notnull" json:"description"`                  // 题目描述
	Category      string `xorm:"'category' varchar(16) notnull" json:"category"`                 // 题目分类
	HasAttachment bool   `xorm:"'has_attachment' bool notnull default(0)" json:"has_attachment"` // 是否有附件
	IsPracticable bool   `xorm:"'is_practicable' bool notnull default(0)" json:"is_practicable"` // 是否为练习题
	IsDynamic     bool   `xorm:"'is_dynamic' bool default(0)" json:"is_dynamic"`                 // 是否为动态题目
	IsEnabled     bool   `xorm:"'is_enabled' bool default(0) notnull" json:"is_enabled"`         // 是否启用
	ExposedPort   int    `xorm:"'exposed_port' int" json:"exposed_port"`                         // 容器暴露端口
	Image         string `xorm:"'image' text" json:"image"`                                      // 题目镜像
	Flag          string `xorm:"'flag' varchar(255)" json:"flag"`                                // Flag 字符串（静态）
	FlagEnv       string `xorm:"'flag_env' varchar(16)" json:"flag_env"`                         // Flag 环境变量（动态）
	FlagFmt       string `xorm:"'flag_fmt' varchar(64)" json:"flag_fmt"`                         // Flag 格式（动态）
	MemoryLimit   int64  `xorm:"'memory_limit' int default(256)" json:"memory_limit"`            // 内存限制（MB）
	Duration      int    `xorm:"'duration' int default(1800)" json:"duration"`                   // 维持时间（秒）
	Difficulty    int    `xorm:"'difficulty' int default(1)" json:"difficulty"`                  // 难度系数（1~5）
	MaxPts        int    `xorm:"'max_pts' int default(1000) notnull" json:"max_pts"`             // 最大得分
	MinPts        int    `xorm:"'min_pts' int default(200) notnull" json:"min_pts"`              // 最小得分
	CreatedAt     int64  `xorm:"created 'created_at'" json:"created_at"`                         // 创建时间
	UpdatedAt     int64  `xorm:"updated 'updated_at'" json:"updated_at"`                         // 更新时间
}
