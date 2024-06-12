package request

type WebhookCreateRequest struct {
	URL    string `json:"url"`
	Type   string `json:"type"`
	Secret string `json:"secret"`
	SSL    *bool  `json:"ssl"`
	GameID *uint  `json:"game_id,omitempty"`
}

type WebhookUpdateRequest struct {
	ID     uint   `json:"id"`
	URL    string `json:"url"`
	Type   string `json:"type"`
	Secret string `json:"secret"`
	SSL    *bool  `json:"ssl"`
	GameID *uint  `json:"game_id,omitempty"`
}

type WebhookDeleteRequest struct {
	ID uint `json:"id"`
}
