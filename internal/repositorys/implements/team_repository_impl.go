package implements

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/repositorys"
	"xorm.io/xorm"
)

type TeamRepositoryImpl struct {
	Db *xorm.Engine
}

func NewTeamRepositoryImpl(Db *xorm.Engine) repositorys.TeamRepository {
	return &TeamRepositoryImpl{Db: Db}
}

func (t TeamRepositoryImpl) Insert(team model.Team) error {
	_, err := t.Db.Table("team").Insert(&team)
	return err
}

func (t TeamRepositoryImpl) Update(team model.Team) error {
	_, err := t.Db.Table("team").ID(team.TeamId).Update(&team)
	return err
}

func (t TeamRepositoryImpl) Delete(id string) error {
	_, err := t.Db.Table("team").ID(id).Delete(&model.Team{})
	return err
}

func (t TeamRepositoryImpl) Find(req request.TeamFindRequest) (teams []model.Team, count int64, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.TeamName != "" {
			q = q.Where("name LIKE ?", "%"+req.TeamName+"%")
		}
		if req.CaptainId != "" {
			q = q.Where("captain_id = ?", req.CaptainId)
		}
		return q
	}
	db := applyFilters(t.Db.Table("team"))
	ct := applyFilters(t.Db.Table("team"))
	count, err = ct.Count(&model.Team{})
	if req.Page != -1 && req.Size != -1 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&teams)
	return teams, count, err
}

func (t TeamRepositoryImpl) FindById(id string) (team model.Team, err error) {
	team = model.Team{}
	has, err := t.Db.Table("team").ID(id).Get(&team)
	if has {
		return team, nil
	} else {
		return team, err
	}
}
