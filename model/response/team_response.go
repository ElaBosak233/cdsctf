package response

import (
	"time"
)

type TeamResponse struct {
	TeamId    string    `json:"team_id"`
	TeamName  string    `json:"team_name"`
	CaptainId string    `json:"captain_id"`
	IsLocked  int       `json:"is_locked"`
	UserIds   []string  `json:"user_ids"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}
