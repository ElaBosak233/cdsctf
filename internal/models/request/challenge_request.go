package request

type ChallengeCreateRequest struct {
	Title         string `json:"title" default:"新题目"`
	Description   string `json:"description" default:"题目描述"`
	Category      string `json:"category" default:"misc"`
	HasAttachment bool   `json:"has_attachment" default:"false"`
	IsPracticable bool   `json:"is_practicable" default:"false"`
	IsDynamic     bool   `json:"is_dynamic" default:"true"`
	ExposedPort   int64  `json:"exposed_port" default:"80"`
	Image         string `json:"image" default:"nginx"`
	Flag          string `json:"flag" default:"PgsCTF{Th4nk5_4_us1ng_PgsHub}"`
	FlagEnv       string `json:"flag_env" default:"FLAG"`
	FlagFmt       string `json:"flag_fmt" default:"PgsCTF{[UUID]}"`
	MemoryLimit   int64  `json:"memory_limit" default:"512"`
	Duration      int64  `json:"duration" default:"1800"`
	Difficulty    int64  `json:"difficulty" default:"1"`
	PracticePts   int64  `json:"practice_pts" default:"200"`
}

type ChallengeUpdateRequest struct {
	ChallengeId   int64  `json:"id"`
	Title         string `json:"title"`
	Description   string `json:"description"`
	Category      string `json:"category"`
	HasAttachment bool   `json:"has_attachment"`
	IsPracticable bool   `json:"is_practicable"`
	IsDynamic     bool   `json:"is_dynamic"`
	ExposedPort   int    `json:"exposed_port"`
	Image         string `json:"image"`
	Flag          string `json:"flag"`
	FlagEnv       string `json:"flag_env"`
	FlagFmt       string `json:"flag_fmt"`
	MemoryLimit   int64  `json:"memory_limit"`
	Duration      int64  `json:"duration"`
	Difficulty    int64  `json:"difficulty"`
	PracticePts   int64  `json:"practice_pts"`
}

type ChallengeDeleteRequest struct {
	ChallengeId int64 `json:"id" binding:"required"`
}

type ChallengeFindRequest struct {
	Id            int64  `json:"id"`
	Category      string `json:"category"`
	Title         string `json:"title"`
	IsPracticable int    `json:"is_practicable"`
	IsDynamic     int    `json:"is_dynamic"`
	IsDetailed    int    `json:"is_detailed"`
	Difficulty    int64  `json:"difficulty"`
	Page          int    `json:"page"`
	Size          int    `json:"size"`
}
