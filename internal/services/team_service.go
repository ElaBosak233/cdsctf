package services

import (
	"errors"
	model "github.com/elabosak233/pgshub/internal/models/data"
	modelm2m "github.com/elabosak233/pgshub/internal/models/data/relations"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/elabosak233/pgshub/internal/repositories/relations"
	"github.com/mitchellh/mapstructure"
	"math"
)

type TeamService interface {
	Create(req request.TeamCreateRequest) error
	Update(req request.TeamUpdateRequest) error
	Delete(id int64) error
	Join(req request.TeamJoinRequest) (err error)
	Quit(req request.TeamQuitRequest) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, err error)
	FindById(id int64) (res response.TeamResponse, err error)
}

type TeamServiceImpl struct {
	UserRepository     repositories.UserRepository
	TeamRepository     repositories.TeamRepository
	UserTeamRepository relations.UserTeamRepository
}

func NewTeamServiceImpl(appRepository *repositories.AppRepository) TeamService {
	return &TeamServiceImpl{
		UserRepository:     appRepository.UserRepository,
		TeamRepository:     appRepository.TeamRepository,
		UserTeamRepository: appRepository.UserTeamRepository,
	}
}

func (t *TeamServiceImpl) Create(req request.TeamCreateRequest) error {
	user, err := t.UserRepository.FindById(req.CaptainId)
	if user.UserId != 0 && err == nil {
		team, err := t.TeamRepository.Insert(model.Team{
			TeamName:  req.TeamName,
			CaptainId: req.CaptainId,
			IsLocked:  false,
		})
		err = t.UserTeamRepository.Insert(modelm2m.UserTeam{
			TeamId: team.TeamId,
			UserId: req.CaptainId,
		})
		return err
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) Update(req request.TeamUpdateRequest) error {
	user, err := t.UserRepository.FindById(req.CaptainId)
	if user.UserId != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamId)
		if team.TeamId != 0 {
			err = t.TeamRepository.Update(model.Team{
				TeamId:    team.TeamId,
				TeamName:  req.TeamName,
				CaptainId: req.CaptainId,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) Delete(id int64) error {
	team, err := t.TeamRepository.FindById(id)
	if team.TeamId != 0 {
		err = t.TeamRepository.Delete(id)
		err = t.UserTeamRepository.DeleteByTeamId(id)
		return err
	} else {
		return errors.New("团队不存在")
	}
}

func (t *TeamServiceImpl) Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, err error) {
	ts, count, err := t.TeamRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	for _, team := range ts {
		res := response.TeamResponse{}
		userTeams, _ := t.UserTeamRepository.FindByTeamId(team.TeamId)
		for _, userTeam := range userTeams {
			res.UserIds = append(res.UserIds, userTeam.UserId)
		}
		_ = mapstructure.Decode(team, &res)
		teams = append(teams, res)
	}
	return teams, pageCount, err
}

func (t *TeamServiceImpl) Join(req request.TeamJoinRequest) error {
	user, err := t.UserRepository.FindById(req.UserId)
	if user.UserId != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamId)
		if team.TeamId != 0 {
			err = t.UserTeamRepository.Insert(modelm2m.UserTeam{
				TeamId: team.TeamId,
				UserId: req.UserId,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) Quit(req request.TeamQuitRequest) (err error) {
	user, err := t.UserRepository.FindById(req.UserId)
	if user.UserId != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamId)
		if team.TeamId != 0 {
			err = t.UserTeamRepository.Delete(modelm2m.UserTeam{
				TeamId: team.TeamId,
				UserId: req.UserId,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) FindById(id int64) (res response.TeamResponse, err error) {
	team, err := t.TeamRepository.FindById(id)
	if team.TeamId != 0 {
		userTeams, _ := t.UserTeamRepository.FindByTeamId(team.TeamId)
		for _, userTeam := range userTeams {
			res.UserIds = append(res.UserIds, userTeam.UserId)
		}
		_ = mapstructure.Decode(team, &res)
		res.CreatedAt = team.CreatedAt
		res.UpdatedAt = team.UpdatedAt
		return res, err
	} else {
		return res, errors.New("团队不存在")
	}
}
