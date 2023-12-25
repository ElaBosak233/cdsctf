package relations

import model "github.com/elabosak233/pgshub/internal/models/data/relations"

type UserTeamRepository interface {
	Insert(userTeam model.UserTeam) error
	Delete(userTeam model.UserTeam) error
	DeleteByUserId(userId string) error
	DeleteByTeamId(teamId string) error
	Exist(userTeam model.UserTeam) (bool, error)
	FindByUserId(userId string) (userTeams []model.UserTeam, err error)
	FindByTeamId(teamId string) (userTeams []model.UserTeam, err error)
	FindAll() (userTeams []model.UserTeam, err error)
}
