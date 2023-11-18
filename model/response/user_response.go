package response

type UserResponse struct {
	Id       string   `json:"id"`
	Username string   `json:"username"`
	GroupIds []string `json:"group_ids"`
}
