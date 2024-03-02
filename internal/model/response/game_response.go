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
