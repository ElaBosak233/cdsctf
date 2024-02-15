package model

type TeamGame struct {
	ID        int64  `json:"id"`
	TeamID    int64  `gorm:"index" json:"team_id"`
	GameID    int64  `gorm:"index" json:"game_id"`
	TeamToken string `gorm:"unique" json:"team_token"`
}
