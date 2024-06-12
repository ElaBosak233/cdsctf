package request

type FlagCreateRequest struct {
	ChallengeID uint   `json:"-"`
	Type        string `json:"type"`
	Banned      *bool  `json:"banned"`
	Value       string `json:"value"`
	Env         string `json:"env"`
}

type FlagUpdateRequest struct {
	ID          uint   `json:"-"`
	ChallengeID uint   `json:"-"`
	Type        string `json:"type"`
	Banned      *bool  `json:"banned"`
	Value       string `json:"value"`
	Env         string `json:"env"`
}

type FlagDeleteRequest struct {
	ID uint `json:"-"`
}
