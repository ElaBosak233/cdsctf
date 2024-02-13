package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IUserTeamRepository interface {
	Insert(userTeam model.UserTeam) error
	Delete(userTeam model.UserTeam) error
	DeleteByUserId(userId int64) error
	DeleteByTeamId(teamId int64) error
	Exist(userTeam model.UserTeam) (bool, error)
	FindByUserId(userId int64) (userTeams []model.UserTeam, err error)
	FindByTeamId(teamId int64) (userTeams []model.UserTeam, err error)
	FindAll() (userTeams []model.UserTeam, err error)
}

type UserTeamRepository struct {
	Db *xorm.Engine
}

func NewUserTeamRepository(Db *xorm.Engine) IUserTeamRepository {
	return &UserTeamRepository{Db: Db}
}

func (t *UserTeamRepository) Insert(userTeam model.UserTeam) error {
	_, err := t.Db.Table("user_team").Insert(&userTeam)
	return err
}

func (t *UserTeamRepository) Delete(userTeam model.UserTeam) error {
	_, err := t.Db.Table("user_team").Delete(&userTeam)
	return err
}

func (t *UserTeamRepository) DeleteByUserId(userId int64) error {
	_, err := t.Db.Table("user_team").Where("user_id = ?", userId).Delete(&model.UserTeam{})
	return err
}

func (t *UserTeamRepository) DeleteByTeamId(teamId int64) error {
	_, err := t.Db.Table("user_team").Where("team_id = ?", teamId).Delete(&model.UserTeam{})
	return err
}

func (t *UserTeamRepository) Exist(userTeam model.UserTeam) (bool, error) {
	r, err := t.Db.Table("user_team").Exist(&userTeam)
	return r, err
}

func (t *UserTeamRepository) FindByUserId(userId int64) (userTeams []model.UserTeam, err error) {
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

func (t *UserTeamRepository) FindByTeamId(teamId int64) (userTeams []model.UserTeam, err error) {
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

func (t *UserTeamRepository) FindAll() (userTeams []model.UserTeam, err error) {
	var userTeam []model.UserTeam
	err = t.Db.Find(&userTeam)
	if err != nil {
		return nil, err
	}
	return userTeam, err
}
