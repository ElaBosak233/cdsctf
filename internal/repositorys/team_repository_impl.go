package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"xorm.io/xorm"
)

type TeamRepositoryImpl struct {
	Db *xorm.Engine
}

func NewTeamRepositoryImpl(Db *xorm.Engine) TeamRepository {
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

func (t TeamRepositoryImpl) FindById(id string) (team model.Team, err error) {
	team = model.Team{}
	has, err := t.Db.Table("team").ID(id).Get(&team)
	if has {
		return team, nil
	} else {
		return team, err
	}
}

func (t TeamRepositoryImpl) FindAll() (teams []model.Team, err error) {
	err = t.Db.Table("team").Find(&teams)
	return teams, err
}
