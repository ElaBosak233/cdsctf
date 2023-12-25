package services

import (
	"errors"
	model "github.com/elabosak233/pgshub/internal/models/data"
	modelm2m "github.com/elabosak233/pgshub/internal/models/data/m2m"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/repositorys/m2m"
	"github.com/google/uuid"
	"github.com/mitchellh/mapstructure"
)

type TeamServiceImpl struct {
	UserRepository     repositorys.UserRepository
	TeamRepository     repositorys.TeamRepository
	UserTeamRepository m2m.UserTeamRepository
}

func NewTeamServiceImpl(appRepository *repositorys.AppRepository) TeamService {
	return &TeamServiceImpl{
		UserRepository:     appRepository.UserRepository,
		TeamRepository:     appRepository.TeamRepository,
		UserTeamRepository: appRepository.UserTeamRepository,
	}
}

func (t *TeamServiceImpl) Create(req request.TeamCreateRequest) error {
	user, err := t.UserRepository.FindById(req.CaptainId)
	uid := uuid.NewString()
	if user.UserId != "" && err == nil {
		err = t.TeamRepository.Insert(model.Team{
			TeamId:    uid,
			TeamName:  req.TeamName,
			CaptainId: req.CaptainId,
			IsLocked:  0,
		})
		err = t.UserTeamRepository.Insert(modelm2m.UserTeam{
			TeamId: uid,
			UserId: req.CaptainId,
		})
		return err
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) Update(req request.TeamUpdateRequest) error {
	user, err := t.UserRepository.FindById(req.CaptainId)
	if user.UserId != "" && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamId)
		if team.TeamId != "" {
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

func (t *TeamServiceImpl) Delete(id string) error {
	team, err := t.TeamRepository.FindById(id)
	if team.TeamId != "" {
		err = t.TeamRepository.Delete(id)
		err = t.UserTeamRepository.DeleteByTeamId(id)
		return err
	} else {
		return errors.New("团队不存在")
	}
}

func (t *TeamServiceImpl) Join(req request.TeamJoinRequest) error {
	user, err := t.UserRepository.FindById(req.UserId)
	if user.UserId != "" && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamId)
		if team.TeamId != "" {
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

func (t *TeamServiceImpl) Quit(req request.TeamQuitRequest) error {
	user, err := t.UserRepository.FindById(req.UserId)
	if user.UserId != "" && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamId)
		if team.TeamId != "" {
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

func (t *TeamServiceImpl) FindById(id string) (res response.TeamResponse, err error) {
	team, err := t.TeamRepository.FindById(id)
	if team.TeamId != "" {
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

func (t *TeamServiceImpl) FindAll() (reses []response.TeamResponse, err error) {
	teams, err := t.TeamRepository.FindAll()
	for _, team := range teams {
		res := response.TeamResponse{}
		userTeams, _ := t.UserTeamRepository.FindByTeamId(team.TeamId)
		for _, userTeam := range userTeams {
			res.UserIds = append(res.UserIds, userTeam.UserId)
		}
		_ = mapstructure.Decode(team, &res)
		res.CreatedAt = team.CreatedAt
		res.UpdatedAt = team.UpdatedAt
		reses = append(reses, res)
	}
	return reses, err
}
