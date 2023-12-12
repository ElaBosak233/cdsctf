package response

import "time"

type UserResponse struct {
	UserId    string    `json:"id"`
	Username  string    `json:"username"`
	Email     string    `json:"email"`
	TeamIds   []string  `json:"team_ids"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}
