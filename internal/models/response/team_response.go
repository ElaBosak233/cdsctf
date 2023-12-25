package response

type TeamResponse struct {
	TeamId    string   `json:"id"`
	TeamName  string   `json:"name"`
	CaptainId string   `json:"captain_id"`
	IsLocked  int      `json:"is_locked"`
	UserIds   []string `json:"user_ids"`
	CreatedAt int64    `json:"created_at"`
	UpdatedAt int64    `json:"updated_at"`
}
