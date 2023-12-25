package response

type UserResponse struct {
	UserId    string   `json:"id"`
	Username  string   `json:"username"`
	Role      int      `json:"role"`
	Email     string   `json:"email"`
	TeamIds   []string `json:"team_ids"`
	CreatedAt int64    `json:"created_at"`
	UpdatedAt int64    `json:"updated_at"`
}
