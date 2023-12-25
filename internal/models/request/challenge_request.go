package request

type ChallengeCreateRequest struct {
	Title         string `json:"title"`
	Description   string `json:"description"`
	HasAttachment bool   `json:"has_attachment"`
	IsPracticable bool   `json:"is_practicable"`
	IsDynamic     bool   `json:"is_dynamic"`
	ExposedPort   int    `json:"exposed_port"`
	Image         string `json:"image"`
	Flag          string `json:"flag"`
	FlagEnv       string `json:"flag_env"`
	FlagPrefix    string `json:"flag_prefix"`
	MemoryLimit   int64  `json:"memory_limit"`
	Duration      int    `json:"duration"`
	Difficulty    int    `json:"difficulty"`
	Category      string `json:"category"`
}

type ChallengeUpdateRequest struct {
	ChallengeId   string `json:"id"`
	Title         string `json:"title"`
	Description   string `json:"description"`
	HasAttachment bool   `json:"has_attachment"`
	IsPracticable bool   `json:"is_practicable"`
	IsDynamic     bool   `json:"is_dynamic"`
	ExposedPort   int    `json:"exposed_port"`
	Image         string `json:"image"`
	Flag          string `json:"flag"`
	FlagEnv       string `json:"flag_env"`
	FlagPrefix    string `json:"flag_prefix"`
	MemoryLimit   int64  `json:"memory_limit"`
	Duration      int    `json:"duration"`
	Difficulty    int    `json:"difficulty"`
	Category      string `json:"category"`
}

type ChallengeDeleteRequest struct {
	ChallengeId string `json:"id" binding:"required"`
}

type ChallengeFindRequest struct {
	Category      string `json:"category"`
	Title         string `json:"title"`
	IsPracticable int    `json:"is_practicable"`
	IsDynamic     int    `json:"is_dynamic"`
	Difficulty    int    `json:"difficulty"`
	Page          int    `json:"page"`
	Size          int    `json:"size"`
}
