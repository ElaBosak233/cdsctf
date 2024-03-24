package model

type GameTeam struct {
	ID        uint   `json:"id"`
	TeamID    uint   `gorm:"uniqueIndex:game_team_idx" json:"team_id"`
	Team      *Team  `json:"team"`
	GameID    uint   `gorm:"uniqueIndex:game_team_idx" json:"game_id"`
	Game      *Game  `json:"game"`
	Rank      int    `json:"rank"`
	Pts       int64  `json:"pts"`
	Solved    int    `json:"solved"`
	IsAllowed *bool  `gorm:"default:false;not null;" json:"is_allowed"`
	Signature string `gorm:"unique" json:"signature"`
}
