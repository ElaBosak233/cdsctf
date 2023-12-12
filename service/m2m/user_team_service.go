package m2m

import model "github.com/elabosak233/pgshub/model/data/m2m"

type UserTeamService interface {
	Insert(userTeam model.UserTeam) error
	Delete(userTeam model.UserTeam) error
	Exist(userTeam model.UserTeam) (bool, error)
	FindByUserId(userId string) (userTeams []model.UserTeam, err error)
	FindByTeamId(teamId string) (userTeams []model.UserTeam, err error)
	FindAll() (userTeams []model.UserTeam, err error)
}
