package repository

import "github.com/elabosak233/pgshub/internal/model/data"

type TeamRepository interface {
	Insert(user data.Team)
	Update(user data.Team)
	Delete(id string)
	FindById(id string) (team data.Team, err error)
	SelectByUserId(userId string) []data.Team
	FindAll() []data.Team
}
