package response

import (
	"github.com/elabosak233/pgshub/models/entity"
)

type GameResponse struct {
	entity.Game `xorm:"extends"`
}

type GameSimpleResponse struct {
	GameId int64  `xorm:"'id'" json:"id"`
	Title  string `xorm:"'title'" json:"title"`
}
