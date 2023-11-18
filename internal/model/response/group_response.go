package response

type GroupResponse struct {
	Id      string   `json:"id"`
	Name    string   `json:"name"`
	UserIds []string `json:"user_ids"`
}
