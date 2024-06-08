package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/google/uuid"
)

type ITeamService interface {
	// Create will create a team with the given request.
	Create(req request.TeamCreateRequest) error

	// Update will update a team with the given id.
	Update(req request.TeamUpdateRequest) error

	// Delete will delete a team with the given id.
	Delete(id uint) error

	// Find will return the teams, total count and error.
	Find(req request.TeamFindRequest) ([]model.Team, int64, error)

	// GetInviteToken will return the invite token of the team.
	GetInviteToken(req request.TeamGetInviteTokenRequest) (string, error)

	// UpdateInviteToken will update the invite token of the team.
	UpdateInviteToken(req request.TeamUpdateInviteTokenRequest) (string, error)
}

type TeamService struct {
	userRepository     repository.IUserRepository
	teamRepository     repository.ITeamRepository
	userTeamRepository repository.IUserTeamRepository
}

func NewTeamService(r *repository.Repository) ITeamService {
	return &TeamService{
		userRepository:     r.UserRepository,
		teamRepository:     r.TeamRepository,
		userTeamRepository: r.UserTeamRepository,
	}
}

func (t *TeamService) Create(req request.TeamCreateRequest) error {
	user, err := t.userRepository.FindById(req.CaptainId)
	if err != nil || user.ID == 0 {
		return errors.New("user.not_found")
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
		return errors.New("team.not_found")
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
		return errors.New("team.not_found")
	}
	err = t.teamRepository.Delete(id)
	return err
}

func (t *TeamService) Find(req request.TeamFindRequest) ([]model.Team, int64, error) {
	teams, total, err := t.teamRepository.Find(req)
	for index, team := range teams {
		team.InviteToken = ""
		teams[index] = team
	}
	return teams, total, err
}

func (t *TeamService) GetInviteToken(req request.TeamGetInviteTokenRequest) (token string, err error) {
	team, err := t.teamRepository.FindById(req.ID)
	if err != nil || team.ID == 0 {
		return "", errors.New("team.not_found")
	}
	return team.InviteToken, err
}

func (t *TeamService) UpdateInviteToken(req request.TeamUpdateInviteTokenRequest) (token string, err error) {
	team, err := t.teamRepository.FindById(req.ID)
	if err != nil || team.ID == 0 {
		return "", errors.New("team.not_found")
	}
	uid := uuid.NewString()
	token = uid[:8] + uid[9:13] + uid[14:18] + uid[19:23] + uid[24:]
	err = t.teamRepository.Update(model.Team{
		ID:          req.ID,
		InviteToken: token,
	})
	return token, err
}
