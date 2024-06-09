package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type IUserTeamService interface {
	Join(req request.TeamUserJoinRequest) error
	Create(req request.TeamUserCreateRequest) error
	Delete(req request.TeamUserDeleteRequest) error
}

type UserTeamService struct {
	userTeamRepository repository.IUserTeamRepository
	teamRepository     repository.ITeamRepository
	userRepository     repository.IUserRepository
}

func NewUserTeamService(r *repository.Repository) IUserTeamService {
	return &UserTeamService{
		userTeamRepository: r.UserTeamRepository,
		teamRepository:     r.TeamRepository,
		userRepository:     r.UserRepository,
	}
}

func (t *UserTeamService) Join(req request.TeamUserJoinRequest) error {
	user, err := t.userRepository.FindByID(req.UserID)
	if err != nil || user.ID == 0 {
		return errors.New("user.not_found")
	}
	team, err := t.teamRepository.FindByID(req.TeamID)
	if err != nil || team.ID == 0 {
		return errors.New("team.not_found")
	}
	if team.InviteToken != req.InviteToken {
		return errors.New("team.join.invalid_token")
	}
	err = t.userTeamRepository.Create(model.UserTeam{
		TeamID: team.ID,
		UserID: req.UserID,
	})
	return err
}

func (t *UserTeamService) Create(req request.TeamUserCreateRequest) error {
	user, err := t.userRepository.FindByID(req.UserID)
	if err != nil || user.ID == 0 {
		return errors.New("user.not_found")
	}
	team, err := t.teamRepository.FindByID(req.TeamID)
	if err != nil || team.ID == 0 {
		return errors.New("team.not_found")
	}
	err = t.userTeamRepository.Create(model.UserTeam{
		TeamID: team.ID,
		UserID: req.UserID,
	})
	return err
}

func (t *UserTeamService) Delete(req request.TeamUserDeleteRequest) error {
	user, err := t.userRepository.FindByID(req.UserID)
	if err != nil || user.ID == 0 {
		return errors.New("user.not_found")
	}
	team, err := t.teamRepository.FindByID(req.TeamID)
	if err != nil || team.ID == 0 {
		return errors.New("team.not_found")
	}
	err = t.userTeamRepository.Delete(model.UserTeam{
		TeamID: team.ID,
		UserID: req.UserID,
	})
	return err
}
