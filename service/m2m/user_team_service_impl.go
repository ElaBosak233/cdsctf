package m2m

import (
	"errors"
	model "github.com/elabosak233/pgshub/model/data/m2m"
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/repository/m2m"
)

type UserTeamServiceImpl struct {
	UserTeamRepository m2m.UserTeamRepository
	UserRepository     repository.UserRepository
	TeamRepository     repository.TeamRepository
}

func NewUserTeamServiceImpl(appRepository *repository.AppRepository) UserTeamService {
	return &UserTeamServiceImpl{
		UserTeamRepository: appRepository.UserTeamRepository,
		UserRepository:     appRepository.UserRepository,
		TeamRepository:     appRepository.TeamRepository,
	}
}

func (t *UserTeamServiceImpl) Insert(userTeam model.UserTeam) error {
	existUser, err := t.UserRepository.FindById(userTeam.UserId)
	if existUser.UserId == "" {
		return errors.New("用户不存在")
	}
	existGroup, err := t.TeamRepository.FindById(userTeam.TeamId)
	if existGroup.TeamId == "" {
		return errors.New("团队不存在")
	}
	exist, err := t.UserTeamRepository.Exist(userTeam)
	if err != nil || exist {
		return errors.New("请勿重复添加")
	}
	err = t.UserTeamRepository.Insert(userTeam)
	return err
}

func (t *UserTeamServiceImpl) Delete(userTeam model.UserTeam) error {
	exist, err := t.Exist(userTeam)
	if err != nil || !exist {
		return errors.New("记录不存在")
	}
	err = t.UserTeamRepository.Delete(userTeam)
	return err
}

func (t *UserTeamServiceImpl) Exist(userTeam model.UserTeam) (bool, error) {
	r, err := t.UserTeamRepository.Exist(userTeam)
	return r, err
}

func (t *UserTeamServiceImpl) FindByUserId(userId string) (userTeams []model.UserTeam, err error) {
	userTeam, err := t.UserTeamRepository.FindByUserId(userId)
	if err != nil {
		return nil, err
	}
	return userTeam, err
}

func (t *UserTeamServiceImpl) FindByTeamId(teamId string) (userTeams []model.UserTeam, err error) {
	userTeam, err := t.UserTeamRepository.FindByTeamId(teamId)
	if err != nil {
		return nil, err
	}
	return userTeam, err
}

func (t *UserTeamServiceImpl) FindAll() (userTeams []model.UserTeam, err error) {
	userTeam, err := t.UserTeamRepository.FindAll()
	if err != nil {
		return nil, err
	}
	return userTeam, err
}
