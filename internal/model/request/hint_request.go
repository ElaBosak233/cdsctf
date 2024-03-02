package request

type HintCreateRequest struct {
	ID          uint   `json:"-"`
	ChallengeID uint   `json:"-"`
	Content     string `json:"content"`
	PublishedAt int64  `json:"published_at"`
}

type HintUpdateRequest struct {
	ID          uint   `json:"-"`
	ChallengeID uint   `json:"-"`
	Content     string `json:"content"`
	PublishedAt int64  `json:"published_at"`
}

type HintDeleteRequest struct {
	ID uint `json:"-"`
}
