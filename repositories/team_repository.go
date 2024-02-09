package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"xorm.io/xorm"
)

type TeamRepository interface {
	Insert(team entity.Team) (te entity.Team, err error)
	Update(team entity.Team) (err error)
	Delete(id int64) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error)
	BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error)
	BatchFindByUserId(req request.TeamBatchFindByUserIdRequest) (teams []response.TeamResponseWithUserId, err error)
	FindById(id int64) (team entity.Team, err error)
}

type TeamRepositoryImpl struct {
	Db *xorm.Engine
}

func NewTeamRepositoryImpl(Db *xorm.Engine) TeamRepository {
	return &TeamRepositoryImpl{Db: Db}
}

func (t *TeamRepositoryImpl) Insert(team entity.Team) (te entity.Team, err error) {
	_, err = t.Db.Table("team").Insert(&team)
	return team, err
}

func (t *TeamRepositoryImpl) Update(team entity.Team) (err error) {
	_, err = t.Db.Table("team").ID(team.ID).Update(&team)
	return err
}

func (t *TeamRepositoryImpl) Delete(id int64) (err error) {
	_, err = t.Db.Table("team").ID(id).Delete(&entity.Team{})
	return err
}

func (t *TeamRepositoryImpl) Find(req request.TeamFindRequest) (teams []response.TeamResponse, count int64, err error) {
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
	count, err = ct.Count(&entity.Team{})
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&teams)
	return teams, count, err
}

func (t *TeamRepositoryImpl) BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error) {
	err = t.Db.Table("team").
		In("team.id", req.ID).
		Find(&teams)
	return teams, err
}

func (t *TeamRepositoryImpl) BatchFindByUserId(req request.TeamBatchFindByUserIdRequest) (teams []response.TeamResponseWithUserId, err error) {
	err = t.Db.Table("team").
		Join("INNER", "user_team", "user_team.team_id = team.id").
		In("user_team.user_id", req.UserID).
		Find(&teams)
	return teams, err
}

func (t *TeamRepositoryImpl) FindById(id int64) (team entity.Team, err error) {
	team = entity.Team{}
	has, err := t.Db.Table("team").ID(id).Get(&team)
	if has {
		return team, nil
	} else {
		return team, err
	}
}
