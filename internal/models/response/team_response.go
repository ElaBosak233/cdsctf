package response

type TeamResponse struct {
	TeamId    int64   `json:"id"`
	TeamName  string  `json:"name"`
	CaptainId int64   `json:"captain_id"`
	IsLocked  int     `json:"is_locked"`
	UserIds   []int64 `json:"user_ids"`
	CreatedAt int64   `json:"created_at"`
	UpdatedAt int64   `json:"updated_at"`
}

type TeamSimpleResponse struct {
	TeamId   int64  `json:"id"`
	TeamName string `json:"name"`
}
