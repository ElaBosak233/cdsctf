package response

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
)

type ChallengeResponse struct {
	model.Challenge `xorm:"extends"`
	Submission      model.Submission `xorm:"extends" json:"-"`
	IsSolved        bool             `xorm:"'is_solved'" json:"is_solved"`
}

type ChallengeSimpleResponse struct {
	ChallengeId int64  `xorm:"'id'" json:"id"`
	Title       string `xorm:"'title'" json:"title"`
	Description string `xorm:"'description'" json:"description"`
	Category    string `xorm:"'category'" json:"category"`
}
