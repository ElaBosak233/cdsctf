package response

import (
	"github.com/elabosak233/pgshub/models/entity"
)

type GameResponse struct {
	entity.Game `xorm:"extends"`
}
