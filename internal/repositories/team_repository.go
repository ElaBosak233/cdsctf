package repositories

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
	"xorm.io/xorm"
)

type TeamRepository interface {
	Insert(team model.Team) (te model.Team, err error)
	Update(team model.Team) (err error)
	Delete(id int64) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error)
	BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error)
	FindById(id int64) (team model.Team, err error)
}

type TeamRepositoryImpl struct {
	Db *xorm.Engine
}

func NewTeamRepositoryImpl(Db *xorm.Engine) TeamRepository {
	return &TeamRepositoryImpl{Db: Db}
}

func (t *TeamRepositoryImpl) Insert(team model.Team) (te model.Team, err error) {
	_, err = t.Db.Table("teams").Insert(&team)
	return team, err
}

func (t *TeamRepositoryImpl) Update(team model.Team) (err error) {
	_, err = t.Db.Table("teams").ID(team.TeamId).Update(&team)
	return err
}

func (t *TeamRepositoryImpl) Delete(id int64) (err error) {
	_, err = t.Db.Table("teams").ID(id).Delete(&model.Team{})
	return err
}

func (t *TeamRepositoryImpl) Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.TeamId != 0 {
			q = q.Where("id = ?", req.TeamId)
		}
		if req.TeamName != "" {
			q = q.Where("name LIKE ?", "%"+req.TeamName+"%")
		}
		if req.CaptainId != 0 {
			q = q.Where("captain_id = ?", req.CaptainId)
		}
		return q
	}
	db := applyFilters(t.Db.Table("teams"))
	ct := applyFilters(t.Db.Table("teams"))
	count, err = ct.Count(&model.Team{})
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	db = db.Select(`teams.*,
					(SELECT json_agg(jsonb_build_object('id', users.id, 'username', users.username, 'name', users.name, 'email', users.email))
					FROM user_team
						JOIN users ON user_team.user_id = users.id
					WHERE user_team.team_id = teams.id) AS users`)
	err = db.Find(&teams)
	return teams, count, err
}

func (t *TeamRepositoryImpl) BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		return q
	}
	db := applyFilters(t.Db.Table("teams"))
	db = db.Select(`teams.*,
					(SELECT json_agg(jsonb_build_object('id', users.id, 'username', users.username, 'name', users.name, 'email', users.email))
					FROM user_team
						JOIN users ON user_team.user_id = users.id
					WHERE user_team.team_id = teams.id) AS users`).In("teams.id", req.TeamId)
	err = db.Find(&teams)
	return teams, err
}

func (t *TeamRepositoryImpl) FindById(id int64) (team model.Team, err error) {
	team = model.Team{}
	has, err := t.Db.Table("teams").ID(id).Get(&team)
	if has {
		return team, nil
	} else {
		return team, err
	}
}
