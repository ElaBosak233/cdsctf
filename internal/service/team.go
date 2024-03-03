package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
	"math"
)

type ITeamService interface {
	Create(req request.TeamCreateRequest) error
	Update(req request.TeamUpdateRequest) error
	Delete(id uint) error
	Join(req request.TeamJoinRequest) (err error)
	Quit(req request.TeamQuitRequest) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, total int64, err error)
	FindById(id uint) (res response.TeamResponse, err error)
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
	if user.ID != 0 && err == nil {
		isLocked := false
		team, err := t.teamRepository.Insert(model.Team{
			Name:        req.Name,
			CaptainID:   req.CaptainId,
			Description: req.Description,
			IsLocked:    &isLocked,
		})
		err = t.userTeamRepository.Insert(model.UserTeam{
			TeamID: team.ID,
			UserID: req.CaptainId,
		})
		return err
	}
	return errors.New("用户不存在")
}

func (t *TeamService) Update(req request.TeamUpdateRequest) error {
	user, err := t.userRepository.FindById(req.CaptainId)
	if user.ID != 0 && err == nil {
		team, err := t.teamRepository.FindById(req.ID)
		if team.ID != 0 {
			if team.ID != req.CaptainId {
				err = t.Join(request.TeamJoinRequest{
					TeamID: team.ID,
					UserID: req.CaptainId,
				})
			}
			err = t.teamRepository.Update(model.Team{
				ID:          team.ID,
				Name:        req.Name,
				Description: req.Description,
				CaptainID:   req.CaptainId,
				IsLocked:    req.IsLocked,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamService) Delete(id uint) error {
	team, err := t.teamRepository.FindById(id)
	if team.ID != 0 {
		err = t.teamRepository.Delete(id)
		err = t.userTeamRepository.DeleteByTeamId(id)
		return err
	} else {
		return errors.New("团队不存在")
	}
}

func (t *TeamService) Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, total int64, err error) {
	teamsData, count, err := t.teamRepository.Find(req)
	for _, team := range teamsData {
		var teamResponse response.TeamResponse
		_ = mapstructure.Decode(team, &teamResponse)
		teams = append(teams, teamResponse)
	}
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return teams, pageCount, count, err
}

func (t *TeamService) Join(req request.TeamJoinRequest) error {
	user, err := t.userRepository.FindById(req.UserID)
	if user.ID != 0 && err == nil {
		team, err := t.teamRepository.FindById(req.TeamID)
		if team.ID != 0 {
			err = t.userTeamRepository.Insert(model.UserTeam{
				TeamID: team.ID,
				UserID: req.UserID,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamService) Quit(req request.TeamQuitRequest) (err error) {
	user, err := t.userRepository.FindById(req.UserID)
	if user.ID != 0 && err == nil {
		team, err := t.teamRepository.FindById(req.TeamID)
		if team.ID != 0 {
			err = t.userTeamRepository.Delete(model.UserTeam{
				TeamID: team.ID,
				UserID: req.UserID,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamService) FindById(id uint) (team response.TeamResponse, err error) {
	teams, _, err := t.teamRepository.Find(request.TeamFindRequest{
		ID: id,
	})
	if len(teams) > 0 {
		var teamData response.TeamResponse
		_ = mapstructure.Decode(teams[0], &teamData)
		team = teamData
	}
	return team, err
}
