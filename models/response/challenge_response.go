package response

import (
	"github.com/elabosak233/pgshub/models/entity"
)

type ChallengeResponse struct {
	entity.Challenge `xorm:"extends"`
	IsSolved         bool  `xorm:"-" json:"is_solved"`
	Pts              int64 `xorm:"-" json:"pts"`
}

type ChallengeSimpleResponse struct {
	ID          int64  `xorm:"'id'" json:"id"`
	Title       string `xorm:"'title'" json:"title"`
	Description string `xorm:"'description'" json:"description"`
	Category    string `xorm:"'category'" json:"category"`
}
