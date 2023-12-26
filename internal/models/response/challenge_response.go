package response

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
)

type ChallengeInfoResponse struct {
	ChallengeId   string `json:"id"`
	Title         string `xorm:"'title' varchar(50) notnull" json:"title"`
	Description   string `xorm:"'description' text notnull" json:"description"`
	Category      string `xorm:"'category' varchar(16) notnull" json:"category"`
	HasAttachment bool   `xorm:"'has_attachment' bool notnull" json:"has_attachment"`
	IsDynamic     bool   `xorm:"'is_dynamic' bool" json:"is_dynamic"`
	Duration      int64  `xorm:"'duration' int" json:"duration"`
	Difficulty    int64  `xorm:"'difficulty' int" json:"difficulty"`
	CreatedAt     int64  `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt     int64  `xorm:"updated 'updated_at'" json:"updated_at"`
}

type ChallengeFullResponse struct {
	model.Challenge
}
