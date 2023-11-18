package repository

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type TeamRepository interface {
	Insert(user model.Team)
	Update(user model.Team)
	Delete(id string)
	FindById(id string) (team model.Team, err error)
	SelectByUserId(userId string) []model.Team
	FindAll() []model.Team
}
