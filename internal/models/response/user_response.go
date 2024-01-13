package response

type UserResponse struct {
	UserId    int64   `json:"id"`
	Username  string  `json:"username"`
	Name      string  `json:"name"`
	Role      int64   `json:"role"`
	Email     string  `json:"email"`
	TeamIds   []int64 `json:"team_ids"`
	CreatedAt int64   `json:"created_at"`
	UpdatedAt int64   `json:"updated_at"`
}

type UserSimpleResponse struct {
	UserId   int64  `json:"id"`
	Username string `json:"username"`
	Name     string `json:"name"`
}
