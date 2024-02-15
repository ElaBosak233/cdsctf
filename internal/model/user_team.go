package model

type UserTeam struct {
	ID     uint `json:"id"`
	UserID uint `gorm:"uniqueIndex:user_team_idx;" json:"user_id"`
	TeamID uint `gorm:"uniqueIndex:user_team_idx;" json:"team_id"`
}
