package request

type ConfigUpdateRequest struct {
	Title string `json:"title,omitempty"`
	Bio   string `json:"bio,omitempty"`
}
