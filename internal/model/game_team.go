package model

type GameTeam struct {
	ID        uint   `json:"id,omitempty"`
	TeamID    uint   `gorm:"uniqueIndex:game_team_idx" json:"team_id,omitempty"`
	Team      *Team  `json:"team,omitempty"`
	GameID    uint   `gorm:"uniqueIndex:game_team_idx" json:"game_id,omitempty"`
	Game      *Game  `json:"game,omitempty"`
	IsAllowed *bool  `gorm:"default:false;not null;" json:"is_allowed,omitempty"`
	Signature string `gorm:"unique" json:"signature,omitempty"`
}
