package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"gorm.io/gorm"
)

type IUserTeamRepository interface {
	Insert(userTeam model.UserTeam) error
	Delete(userTeam model.UserTeam) error
	DeleteByUserId(userID uint) error
	DeleteByTeamId(teamID uint) error
	FindByUserId(userID uint) (userTeams []model.UserTeam, err error)
	FindByTeamId(teamId uint) (userTeams []model.UserTeam, err error)
	FindAll() (userTeams []model.UserTeam, err error)
}

type UserTeamRepository struct {
	Db *gorm.DB
}

func NewUserTeamRepository(Db *gorm.DB) IUserTeamRepository {
	return &UserTeamRepository{Db: Db}
}

func (t *UserTeamRepository) Insert(userTeam model.UserTeam) error {
	result := t.Db.Table("user_teams").Create(&userTeam)
	return result.Error
}

func (t *UserTeamRepository) Delete(userTeam model.UserTeam) error {
	result := t.Db.Table("user_teams").Delete(&userTeam)
	return result.Error
}

func (t *UserTeamRepository) DeleteByUserId(userId uint) error {
	result := t.Db.Table("user_teams").Where("user_id = ?", userId).Delete(&model.UserTeam{})
	return result.Error
}

func (t *UserTeamRepository) DeleteByTeamId(teamID uint) error {
	result := t.Db.Table("user_teams").Where("team_id = ?", teamID).Delete(&model.UserTeam{})
	return result.Error
}

func (t *UserTeamRepository) FindByUserId(userId uint) (userTeams []model.UserTeam, err error) {
	result := t.Db.Table("user_teams").
		Joins("INNER JOIN teams ON user_teams.team_id = teams.id").
		Where("user_teams.user_id = ?", userId).
		Find(&userTeams)
	return userTeams, result.Error
}

func (t *UserTeamRepository) FindByTeamId(teamId uint) (userTeams []model.UserTeam, err error) {
	result := t.Db.Table("user_teams").
		Joins("INNER JOIN users ON user_teams.user_id = users.id").
		Where("user_teams.team_id = ?", teamId).
		Find(&userTeams)
	return userTeams, result.Error
}

func (t *UserTeamRepository) FindAll() (userTeams []model.UserTeam, err error) {
	result := t.Db.Table("user_teams").Find(&userTeams)
	return userTeams, result.Error
}
