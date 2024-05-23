package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IGameTeamRepository interface {
	Create(gameTeam model.GameTeam) (err error)
	Update(gameTeam model.GameTeam) (err error)
	Delete(gameTeam model.GameTeam) (err error)
	Find(gameTeam model.GameTeam) (gameTeams []model.GameTeam, total int64, err error)
}

type GameTeamRepository struct {
	db *gorm.DB
}

func NewGameTeamRepository(db *gorm.DB) IGameTeamRepository {
	return &GameTeamRepository{db: db}
}

func (g *GameTeamRepository) Create(gameTeam model.GameTeam) (err error) {
	result := g.db.Table("game_teams").Create(&gameTeam)
	return result.Error
}

func (g *GameTeamRepository) Delete(gameTeam model.GameTeam) (err error) {
	result := g.db.Table("game_teams").
		Where("game_id = ?", gameTeam.GameID).
		Where("team_id = ?", gameTeam.TeamID).
		Delete(&gameTeam)
	return result.Error
}

func (g *GameTeamRepository) Update(gameTeam model.GameTeam) (err error) {
	result := g.db.Table("game_teams").
		Where("game_id = ?", gameTeam.GameID).
		Where("team_id = ?", gameTeam.TeamID).
		Model(&gameTeam).
		Updates(&gameTeam)
	return result.Error
}

func (g *GameTeamRepository) Find(gameTeam model.GameTeam) (gameTeams []model.GameTeam, total int64, err error) {
	db := g.db.Table("game_teams").
		Where(&gameTeam)

	result := db.Model(&model.GameTeam{}).Count(&total)

	result = db.Preload("Team", func(db *gorm.DB) *gorm.DB {
		return db.Preload("Captain", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "nickname", "username", "email"})
		}).Preload("Users", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "nickname", "username", "email"})
		})
	}).
		Preload("Game", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "title", "started_at", "ended_at"})
		}).
		Find(&gameTeams)

	return gameTeams, total, result.Error
}
