package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
)

type TeamRepository interface {
	Insert(team model.Team) error
	Update(team model.Team) error
	Delete(id string) error
	FindById(id string) (team model.Team, err error)
	FindAll() (teams []model.Team, err error)
}
