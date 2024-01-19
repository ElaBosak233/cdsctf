package relations

import (
	model "github.com/elabosak233/pgshub/models/entity/relations"
	"github.com/xormplus/xorm"
)

type UserTeamRepository interface {
	Insert(userTeam model.UserTeam) error
	Delete(userTeam model.UserTeam) error
	DeleteByUserId(userId int64) error
	DeleteByTeamId(teamId int64) error
	Exist(userTeam model.UserTeam) (bool, error)
	FindByUserId(userId int64) (userTeams []model.UserTeam, err error)
	FindByTeamId(teamId int64) (userTeams []model.UserTeam, err error)
	FindAll() (userTeams []model.UserTeam, err error)
}

type UserTeamRepositoryImpl struct {
	Db *xorm.Engine
}

func NewUserTeamRepositoryImpl(Db *xorm.Engine) UserTeamRepository {
	return &UserTeamRepositoryImpl{Db: Db}
}

func (t *UserTeamRepositoryImpl) Insert(userTeam model.UserTeam) error {
	_, err := t.Db.Table("user_team").Insert(&userTeam)
	return err
}

func (t *UserTeamRepositoryImpl) Delete(userTeam model.UserTeam) error {
	_, err := t.Db.Table("user_team").Delete(&userTeam)
	return err
}

func (t *UserTeamRepositoryImpl) DeleteByUserId(userId int64) error {
	_, err := t.Db.Table("user_team").Where("user_id = ?", userId).Delete(&model.UserTeam{})
	return err
}

func (t *UserTeamRepositoryImpl) DeleteByTeamId(teamId int64) error {
	_, err := t.Db.Table("user_team").Where("team_id = ?", teamId).Delete(&model.UserTeam{})
	return err
}

func (t *UserTeamRepositoryImpl) Exist(userTeam model.UserTeam) (bool, error) {
	r, err := t.Db.Table("user_team").Exist(&userTeam)
	return r, err
}

func (t *UserTeamRepositoryImpl) FindByUserId(userId int64) (userTeams []model.UserTeam, err error) {
	var userTeam []model.UserTeam
	err = t.Db.Table("user_team").
		Join("INNER", "teams", "user_team.team_id = teams.id").
		Where("user_team.user_id = ?", userId).
		Find(&userTeam)
	if err != nil {
		return nil, err
	}
	return userTeam, err
}

func (t *UserTeamRepositoryImpl) FindByTeamId(teamId int64) (userTeams []model.UserTeam, err error) {
	var teamUser []model.UserTeam
	err = t.Db.Table("user_team").
		Join("INNER", "users", "user_team.user_id = users.id").
		Where("user_team.team_id = ?", teamId).
		Find(&teamUser)
	if err != nil {
		return nil, err
	}
	return teamUser, err
}

func (t *UserTeamRepositoryImpl) FindAll() (userTeams []model.UserTeam, err error) {
	var userTeam []model.UserTeam
	err = t.Db.Find(&userTeam)
	if err != nil {
		return nil, err
	}
	return userTeam, err
}
