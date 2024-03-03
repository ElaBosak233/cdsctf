package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type GameResponse struct {
	model.Game `xorm:"extends"`
}

type GameSimpleResponse struct {
	ID    int64  `xorm:"'id'" json:"id"`
	Title string `xorm:"'title'" json:"title"`
}

type GameChallengeResponse struct {
	*model.Challenge
	IsEnabled bool  `json:"is_enabled"`
	MaxPts    int   `json:"max_pts"`
	MinPts    int   `json:"min_pts"`
	Pts       int64 `json:"pts"`
}
