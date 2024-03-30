package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"math"
)

type ITeamService interface {
	Create(req request.TeamCreateRequest) error
	Update(req request.TeamUpdateRequest) error
	Delete(id uint) error
	Find(req request.TeamFindRequest) (teams []model.Team, pageCount int64, total int64, err error)
	FindById(id uint) (team model.Team, err error)
}

type TeamService struct {
	userRepository     repository.IUserRepository
	teamRepository     repository.ITeamRepository
	userTeamRepository repository.IUserTeamRepository
}

func NewTeamService(appRepository *repository.Repository) ITeamService {
	return &TeamService{
		userRepository:     appRepository.UserRepository,
		teamRepository:     appRepository.TeamRepository,
		userTeamRepository: appRepository.UserTeamRepository,
	}
}

func (t *TeamService) Create(req request.TeamCreateRequest) error {
	user, err := t.userRepository.FindById(req.CaptainId)
	if err != nil || user.ID == 0 {
		return errors.New("用户不存在")
	}
	isLocked := false
	team, err := t.teamRepository.Insert(model.Team{
		Name:        req.Name,
		CaptainID:   req.CaptainId,
		Description: req.Description,
		Email:       req.Email,
		IsLocked:    &isLocked,
	})
	err = t.userTeamRepository.Insert(model.UserTeam{
		TeamID: team.ID,
		UserID: req.CaptainId,
	})
	return err
}

func (t *TeamService) Update(req request.TeamUpdateRequest) error {
	team, err := t.teamRepository.FindById(req.ID)
	if err != nil || team.ID == 0 {
		return errors.New("团队不存在")
	}
	err = t.teamRepository.Update(model.Team{
		ID:          team.ID,
		Name:        req.Name,
		Description: req.Description,
		CaptainID:   req.CaptainId,
		Email:       req.Email,
		IsLocked:    req.IsLocked,
	})
	return err
}

func (t *TeamService) Delete(id uint) error {
	team, err := t.teamRepository.FindById(id)
	if err != nil || team.ID == 0 {
		return errors.New("团队不存在")
	}
	err = t.teamRepository.Delete(id)
	err = t.userTeamRepository.DeleteByTeamId(id)
	return err
}

func (t *TeamService) Find(req request.TeamFindRequest) (teams []model.Team, pageCount int64, total int64, err error) {
	teams, count, err := t.teamRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return teams, pageCount, count, err
}

func (t *TeamService) FindById(id uint) (team model.Team, err error) {
	teams, _, err := t.teamRepository.Find(request.TeamFindRequest{
		ID: id,
	})
	if len(teams) > 0 {
		team = teams[0]
	}
	return team, err
}
