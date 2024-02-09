package request

type FlagCreateRequest struct {
	Type  string `json:"type"`
	Value string `json:"value"`
	Env   string `json:"env"`
}
