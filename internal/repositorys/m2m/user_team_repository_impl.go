package m2m

import (
	model "github.com/elabosak233/pgshub/internal/models/data/m2m"
	"xorm.io/xorm"
)

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

func (t *UserTeamRepositoryImpl) DeleteByUserId(userId string) error {
	_, err := t.Db.Table("user_team").Where("user_id = ?", userId).Delete(&model.UserTeam{})
	return err
}

func (t *UserTeamRepositoryImpl) DeleteByTeamId(teamId string) error {
	_, err := t.Db.Table("user_team").Where("team_id = ?", teamId).Delete(&model.UserTeam{})
	return err
}

func (t *UserTeamRepositoryImpl) Exist(userTeam model.UserTeam) (bool, error) {
	r, err := t.Db.Table("user_team").Exist(&userTeam)
	return r, err
}

func (t *UserTeamRepositoryImpl) FindByUserId(userId string) (userTeams []model.UserTeam, err error) {
	var userTeam []model.UserTeam
	err = t.Db.Table("user_team").
		Join("INNER", "team", "user_team.team_id = team.id").
		Where("user_team.user_id = ?", userId).
		Find(&userTeam)
	if err != nil {
		return nil, err
	}
	return userTeam, err
}

func (t *UserTeamRepositoryImpl) FindByTeamId(teamId string) (userTeams []model.UserTeam, err error) {
	var teamUser []model.UserTeam
	err = t.Db.Table("user_team").
		Join("INNER", "user", "user_team.user_id = user.id").
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
