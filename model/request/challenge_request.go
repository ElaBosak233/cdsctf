package request

type ChallengeCreateRequest struct {
	Title        string `xorm:"varchar(50) 'title' notnull" json:"title"`
	Description  string `xorm:"text 'description' notnull" json:"description"`
	AttachmentId string `xorm:"json 'attachment_ids'" json:"attachment_id"`
	IsDynamic    int    `xorm:"'is_dynamic' int" json:"is_dynamic" binding:"oneof=0 1"`
	ExposedPort  int    `xorm:"'exposed_port' int" json:"exposed_port"`
	Image        string `xorm:"'image' text" json:"image"`
	Flag         string `xorm:"'flag' text" json:"flag"`
	FlagEnv      string `xorm:"'flag_env' text" json:"flag_env"`
	MemoryLimit  int64  `xorm:"'memory_limit' int" json:"memory_limit"`
	Duration     int    `xorm:"'duration' int" json:"duration"`
	Difficulty   int    `xorm:"int 'difficulty'" json:"difficulty"`
}

type ChallengeUpdateRequest struct {
	ChallengeId  string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	Title        string `xorm:"varchar(50) 'title' notnull" json:"title"`
	Description  string `xorm:"text 'description' notnull" json:"description"`
	AttachmentId string `xorm:"json 'attachment_ids'" json:"attachment_id"`
	IsDynamic    int    `xorm:"'is_dynamic' int" json:"is_dynamic" binding:"oneof=0 1"`
	ExposedPort  int    `xorm:"'exposed_port' int" json:"exposed_port"`
	Image        string `xorm:"'image' text" json:"image"`
	Flag         string `xorm:"'flag' text" json:"flag"`
	FlagEnv      string `xorm:"'flag_env' text" json:"flag_env"`
	MemoryLimit  int64  `xorm:"'memory_limit' int" json:"memory_limit"`
	Duration     int    `xorm:"'duration' int" json:"duration"`
	Difficulty   int    `xorm:"int 'difficulty'" json:"difficulty"`
}

type ChallengeDeleteRequest struct {
	ChallengeId string `json:"id" binding:"required"`
}
