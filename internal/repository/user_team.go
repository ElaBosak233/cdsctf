package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IUserTeamRepository interface {
	Create(userTeam model.UserTeam) error
	Delete(userTeam model.UserTeam) error
}

type UserTeamRepository struct {
	db *gorm.DB
}

func NewUserTeamRepository(db *gorm.DB) IUserTeamRepository {
	return &UserTeamRepository{db: db}
}

func (t *UserTeamRepository) Create(userTeam model.UserTeam) error {
	result := t.db.Table("user_teams").Create(&userTeam)
	return result.Error
}

func (t *UserTeamRepository) Delete(userTeam model.UserTeam) error {
	result := t.db.Table("user_teams").
		Where("user_id = ?", userTeam.UserID).
		Where("team_id = ?", userTeam.TeamID).
		Delete(&userTeam)
	return result.Error
}
