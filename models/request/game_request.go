package request

type GameFindRequest struct {
	GameId    int64    `json:"id"`
	Title     string   `json:"title"`
	SortBy    []string `json:"sort_by"`
	IsEnabled int      `json:"is_enabled"`
	Page      int      `json:"page"`
	Size      int      `json:"size"`
}

type GameCreateRequest struct {
	Title                  string  `json:"title" binding:"required" msg:"标题不能为空"`
	Bio                    string  `json:"bio"`
	Description            string  `json:"description"`
	IsEnabled              *bool   `json:"is_enabled"`
	IsPublic               *bool   `json:"is_public"`
	Password               string  `json:"password"`
	MemberLimitMin         int64   `json:"member_limit_min"`
	MemberLimitMax         int64   `json:"member_limit_max"`
	ParallelContainerLimit int64   `json:"parallel_container_limit"`
	FirstBloodRewardRatio  float64 `json:"first_blood_reward_ratio"`
	SecondBloodRewardRatio float64 `json:"second_blood_reward_ratio"`
	ThirdBloodRewardRatio  float64 `json:"third_blood_reward_ratio"`
	IsNeedWriteUp          *bool   `json:"is_need_write_up"`
	StartedAt              int64   `json:"started_at"`
	EndedAt                int64   `json:"ended_at"`
}

type GameUpdateRequest struct {
	GameId                 int64   `json:"id"`
	Title                  string  `json:"title"`
	Bio                    string  `json:"bio"`
	Description            string  `json:"description"`
	IsEnabled              *bool   `json:"is_enabled"`
	IsPublic               *bool   `json:"is_public"`
	Password               string  `json:"password"`
	MemberLimitMin         int64   `json:"member_limit_min"`
	MemberLimitMax         int64   `json:"member_limit_max"`
	ParallelContainerLimit int64   `json:"parallel_container_limit"`
	FirstBloodRewardRatio  float64 `json:"first_blood_reward_ratio"`
	SecondBloodRewardRatio float64 `json:"second_blood_reward_ratio"`
	ThirdBloodRewardRatio  float64 `json:"third_blood_reward_ratio"`
	IsNeedWriteUp          *bool   `json:"is_need_write_up"`
	StartedAt              int64   `json:"started_at"`
	EndedAt                int64   `json:"ended_at"`
}
