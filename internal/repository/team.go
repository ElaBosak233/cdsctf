package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type ITeamRepository interface {
	Insert(team model.Team) (te model.Team, err error)
	Update(team model.Team) (err error)
	Delete(id uint) (err error)
	Find(req request.TeamFindRequest) (teams []model.Team, count int64, err error)
	FindById(id uint) (team model.Team, err error)
}

type TeamRepository struct {
	db *gorm.DB
}

func NewTeamRepository(db *gorm.DB) ITeamRepository {
	return &TeamRepository{db: db}
}

func (t *TeamRepository) Insert(team model.Team) (te model.Team, err error) {
	result := t.db.Table("teams").Create(&team)
	return team, result.Error
}

func (t *TeamRepository) Update(team model.Team) (err error) {
	result := t.db.Table("teams").Model(&team).Updates(&team)
	return result.Error
}

func (t *TeamRepository) Delete(id uint) (err error) {
	result := t.db.Table("teams").Delete(&model.Team{
		ID: id,
	})
	return result.Error
}

func (t *TeamRepository) Find(req request.TeamFindRequest) (teams []model.Team, count int64, err error) {
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

	result := db.Model(&model.Team{}).Count(&count)
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
	return teams, count, result.Error
}

func (t *TeamRepository) FindById(id uint) (team model.Team, err error) {
	result := t.db.Table("teams").Where("id = ?", id).First(&team)
	return team, result.Error
}
