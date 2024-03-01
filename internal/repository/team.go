package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"gorm.io/gorm"
)

type ITeamRepository interface {
	Insert(team model.Team) (te model.Team, err error)
	Update(team model.Team) (err error)
	Delete(id uint) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error)
	BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error)
	BatchFindByUserId(req request.TeamBatchFindByUserIdRequest) (teams []response.TeamResponseWithUserId, err error)
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
	result := t.db.Table("team").Model(&team).Updates(&team)
	return result.Error
}

func (t *TeamRepository) Delete(id uint) (err error) {
	result := t.db.Table("teams").Delete(&model.Team{
		ID: id,
	})
	return result.Error
}

func (t *TeamRepository) Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error) {
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.TeamName != "" {
			q = q.Where("name LIKE ?", "%"+req.TeamName+"%")
		}
		if req.CaptainID != 0 {
			q = q.Where("captain_id = ?", req.CaptainID)
		}
		return q
	}
	db := applyFilters(t.db.Table("teams"))

	result := db.Model(&model.Team{}).Count(&count)
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	result = db.Find(&teams)
	return teams, count, result.Error
}

func (t *TeamRepository) BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error) {
	result := t.db.Table("teams").
		Where("teams.id IN ?", req.ID).
		Find(&teams)
	return teams, result.Error
}

func (t *TeamRepository) BatchFindByUserId(req request.TeamBatchFindByUserIdRequest) (teams []response.TeamResponseWithUserId, err error) {
	result := t.db.Table("teams").
		Joins("INNER JOIN user_teams ON user_teams.team_id = teams.id").
		Where("user_teams.user_id = ?", req.UserID).
		Find(&teams)
	return teams, result.Error
}

func (t *TeamRepository) FindById(id uint) (team model.Team, err error) {
	result := t.db.Table("teams").Where("id = ?", id).First(&team)
	return team, result.Error
}
