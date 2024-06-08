package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type ITeamRepository interface {
	Create(team model.Team) (model.Team, error)
	Update(team model.Team) error
	Delete(id uint) error
	Find(req request.TeamFindRequest) ([]model.Team, int64, error)
	FindById(id uint) (model.Team, error)
}

type TeamRepository struct {
	db *gorm.DB
}

func NewTeamRepository(db *gorm.DB) ITeamRepository {
	return &TeamRepository{db: db}
}

func (t *TeamRepository) Create(team model.Team) (model.Team, error) {
	result := t.db.Table("teams").Create(&team)
	return team, result.Error
}

func (t *TeamRepository) Update(team model.Team) error {
	result := t.db.Table("teams").Model(&team).Updates(&team)
	return result.Error
}

func (t *TeamRepository) Delete(id uint) error {
	result := t.db.Table("teams").Where("id = ?", id).Delete(&model.Team{
		ID: id,
	})
	return result.Error
}

func (t *TeamRepository) Find(req request.TeamFindRequest) ([]model.Team, int64, error) {
	var teams []model.Team
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.Name != "" {
			q = q.Where("name LIKE ?", "%"+req.Name+"%")
		}
		if req.CaptainID != 0 {
			q = q.Where("captain_id = ?", req.CaptainID)
		}
		if req.GameID != nil {
			q = q.Joins("INNER JOIN game_teams ON game_teams.team_id = teams.id").
				Where("game_teams.game_id = ?", *(req.GameID))
		}
		if req.UserID != nil {
			q = q.Joins("INNER JOIN user_teams ON user_teams.team_id = teams.id").
				Where("user_teams.user_id = ?", *(req.UserID))
		}
		return q
	}
	db := applyFilters(t.db.Table("teams"))
	var total int64 = 0
	result := db.Model(&model.Team{}).Count(&total)
	if req.SortKey != "" && req.SortOrder != "" {
		db = db.Order(req.SortKey + " " + req.SortOrder)
	} else {
		db = db.Order("teams.id ASC")
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	result = db.
		Preload("Captain", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "nickname", "username", "email"})
		}).
		Preload("Users", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "nickname", "username", "email"})
		}).
		Find(&teams)
	return teams, total, result.Error
}

func (t *TeamRepository) FindById(id uint) (model.Team, error) {
	var team model.Team
	result := t.db.Table("teams").Where("id = ?", id).First(&team)
	return team, result.Error
}
