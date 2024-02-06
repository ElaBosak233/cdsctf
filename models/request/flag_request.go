package request

type FlagCreateRequest struct {
	Type    string `json:"type"`
	Content string `json:"content"`
	Env     string `json:"env"`
}
