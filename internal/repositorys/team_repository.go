package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
)

type TeamRepository interface {
	Insert(team model.Team) error
	Update(team model.Team) error
	Delete(id string) error
	Find(req request.TeamFindRequest) (teams []model.Team, count int64, err error)
	FindById(id string) (team model.Team, err error)
}
