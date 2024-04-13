package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/google/uuid"
	"math"
)

type ITeamService interface {
	Create(req request.TeamCreateRequest) error
	Update(req request.TeamUpdateRequest) error
	Delete(id uint) error
	Find(req request.TeamFindRequest) (teams []model.Team, pages int64, total int64, err error)
	FindById(id uint) (team model.Team, err error)
	GetInviteToken(req request.TeamGetInviteTokenRequest) (token string, err error)
	UpdateInviteToken(req request.TeamUpdateInviteTokenRequest) (token string, err error)
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
	uid := uuid.NewString()
	_, err = t.teamRepository.Create(model.Team{
		Name:        req.Name,
		CaptainID:   req.CaptainId,
		Description: req.Description,
		Email:       req.Email,
		IsLocked:    &isLocked,
		InviteToken: uid[:8] + uid[9:13] + uid[14:18] + uid[19:23] + uid[24:],
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
	return err
}

func (t *TeamService) Find(req request.TeamFindRequest) (teams []model.Team, pages int64, total int64, err error) {
	teams, count, err := t.teamRepository.Find(req)
	for index, team := range teams {
		team.InviteToken = ""
		teams[index] = team
	}
	if req.Size >= 1 && req.Page >= 1 {
		pages = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pages = 1
	}
	return teams, pages, count, err
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

func (t *TeamService) GetInviteToken(req request.TeamGetInviteTokenRequest) (token string, err error) {
	team, err := t.teamRepository.FindById(req.ID)
	if err != nil || team.ID == 0 {
		return "", errors.New("团队不存在")
	}
	return team.InviteToken, err
}

func (t *TeamService) UpdateInviteToken(req request.TeamUpdateInviteTokenRequest) (token string, err error) {
	team, err := t.teamRepository.FindById(req.ID)
	if err != nil || team.ID == 0 {
		return "", errors.New("团队不存在")
	}
	uid := uuid.NewString()
	token = uid[:8] + uid[9:13] + uid[14:18] + uid[19:23] + uid[24:]
	err = t.teamRepository.Update(model.Team{
		ID:          req.ID,
		InviteToken: token,
	})
	return token, err
}
