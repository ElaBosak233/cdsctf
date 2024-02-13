package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	"xorm.io/xorm"
)

type ITeamRepository interface {
	Insert(team model.Team) (te model.Team, err error)
	Update(team model.Team) (err error)
	Delete(id int64) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error)
	BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error)
	BatchFindByUserId(req request.TeamBatchFindByUserIdRequest) (teams []response.TeamResponseWithUserId, err error)
	FindById(id int64) (team model.Team, err error)
}

type TeamRepository struct {
	Db *xorm.Engine
}

func NewTeamRepository(Db *xorm.Engine) ITeamRepository {
	return &TeamRepository{Db: Db}
}

func (t *TeamRepository) Insert(team model.Team) (te model.Team, err error) {
	_, err = t.Db.Table("team").Insert(&team)
	return team, err
}

func (t *TeamRepository) Update(team model.Team) (err error) {
	_, err = t.Db.Table("team").ID(team.ID).Update(&team)
	return err
}

func (t *TeamRepository) Delete(id int64) (err error) {
	_, err = t.Db.Table("team").ID(id).Delete(&model.Team{})
	return err
}

func (t *TeamRepository) Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.TeamName != "" {
			q = q.Where("name LIKE ?", "%"+req.TeamName+"%")
		}
		if req.CaptainId != 0 {
			q = q.Where("captain_id = ?", req.CaptainId)
		}
		return q
	}
	db := applyFilters(t.Db.Table("team"))
	ct := applyFilters(t.Db.Table("team"))
	count, err = ct.Count(&model.Team{})
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&teams)
	return teams, count, err
}

func (t *TeamRepository) BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error) {
	err = t.Db.Table("team").
		In("team.id", req.ID).
		Find(&teams)
	return teams, err
}

func (t *TeamRepository) BatchFindByUserId(req request.TeamBatchFindByUserIdRequest) (teams []response.TeamResponseWithUserId, err error) {
	err = t.Db.Table("team").
		Join("INNER", "user_team", "user_team.team_id = team.id").
		In("user_team.user_id", req.UserID).
		Find(&teams)
	return teams, err
}

func (t *TeamRepository) FindById(id int64) (team model.Team, err error) {
	team = model.Team{}
	has, err := t.Db.Table("team").ID(id).Get(&team)
	if has {
		return team, nil
	} else {
		return team, err
	}
}
