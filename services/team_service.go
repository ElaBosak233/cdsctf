package services

import (
	"errors"
	"github.com/elabosak233/pgshub/models/entity"
	modelm2m "github.com/elabosak233/pgshub/models/entity/relations"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/repositories/relations"
	"github.com/mitchellh/mapstructure"
	"math"
)

type TeamService interface {
	Create(req request.TeamCreateRequest) error
	Update(req request.TeamUpdateRequest) error
	Delete(id int64) error
	Join(req request.TeamJoinRequest) (err error)
	Quit(req request.TeamQuitRequest) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, total int64, err error)
	BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error)
	FindById(id int64) (res response.TeamResponse, err error)
}

type TeamServiceImpl struct {
	UserRepository     repositories.UserRepository
	TeamRepository     repositories.TeamRepository
	UserTeamRepository relations.UserTeamRepository
}

func NewTeamServiceImpl(appRepository *repositories.Repositories) TeamService {
	return &TeamServiceImpl{
		UserRepository:     appRepository.UserRepository,
		TeamRepository:     appRepository.TeamRepository,
		UserTeamRepository: appRepository.UserTeamRepository,
	}
}

// Mixin 用于向 Team 响应实体中混入 User 实体
func (t *TeamServiceImpl) Mixin(teams []response.TeamResponse) (ts []response.TeamResponse, err error) {
	var teamIds []int64
	teamsMap := make(map[int64]response.TeamResponse)
	for _, team := range teams {
		if _, ok := teamsMap[team.ID]; !ok {
			teamsMap[team.ID] = team
		}
		teamIds = append(teamIds, team.ID)
	}
	users, err := t.UserRepository.BatchFindByTeamId(request.UserBatchFindByTeamIdRequest{
		TeamID: teamIds,
	})
	for _, user := range users {
		var userResponse response.UserResponse
		_ = mapstructure.Decode(user, &userResponse)
		if team, ok := teamsMap[user.TeamId]; ok {
			if team.CaptainId == user.ID {
				team.Captain = userResponse
			}
			team.Users = append(team.Users, userResponse)
			teamsMap[user.TeamId] = team
		}
	}
	for index, team := range teams {
		teams[index].Users = teamsMap[team.ID].Users
		teams[index].Captain = teamsMap[team.ID].Captain
	}
	return teams, err
}

func (t *TeamServiceImpl) Create(req request.TeamCreateRequest) error {
	user, err := t.UserRepository.FindById(req.CaptainId)
	if user.ID != 0 && err == nil {
		isLocked := false
		team, err := t.TeamRepository.Insert(entity.Team{
			Name:        req.Name,
			CaptainID:   req.CaptainId,
			Description: req.Description,
			IsLocked:    &isLocked,
		})
		err = t.UserTeamRepository.Insert(modelm2m.UserTeam{
			TeamId: team.ID,
			UserId: req.CaptainId,
		})
		return err
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) Update(req request.TeamUpdateRequest) error {
	user, err := t.UserRepository.FindById(req.CaptainId)
	if user.ID != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.ID)
		if team.ID != 0 {
			if team.ID != req.CaptainId {
				err = t.Join(request.TeamJoinRequest{
					TeamID: team.ID,
					UserID: req.CaptainId,
				})
			}
			err = t.TeamRepository.Update(entity.Team{
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

func (t *TeamServiceImpl) Delete(id int64) error {
	team, err := t.TeamRepository.FindById(id)
	if team.ID != 0 {
		err = t.TeamRepository.Delete(id)
		err = t.UserTeamRepository.DeleteByTeamId(id)
		return err
	} else {
		return errors.New("团队不存在")
	}
}

func (t *TeamServiceImpl) Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, total int64, err error) {
	teams, count, err := t.TeamRepository.Find(req)
	teams, err = t.Mixin(teams)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return teams, pageCount, count, err
}

func (t *TeamServiceImpl) BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error) {
	teams, err = t.TeamRepository.BatchFind(req)
	teams, err = t.Mixin(teams)
	return teams, err
}

func (t *TeamServiceImpl) Join(req request.TeamJoinRequest) error {
	user, err := t.UserRepository.FindById(req.UserID)
	if user.ID != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamID)
		if team.ID != 0 {
			err = t.UserTeamRepository.Insert(modelm2m.UserTeam{
				TeamId: team.ID,
				UserId: req.UserID,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) Quit(req request.TeamQuitRequest) (err error) {
	user, err := t.UserRepository.FindById(req.UserID)
	if user.ID != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamID)
		if team.ID != 0 {
			err = t.UserTeamRepository.Delete(modelm2m.UserTeam{
				TeamId: team.ID,
				UserId: req.UserID,
			})
			return err
		} else {
			return errors.New("团队不存在")
		}
	}
	return errors.New("用户不存在")
}

func (t *TeamServiceImpl) FindById(id int64) (team response.TeamResponse, err error) {
	teams, _, err := t.TeamRepository.Find(request.TeamFindRequest{
		ID: id,
	})
	if len(teams) > 0 {
		team = teams[0]
	}
	return team, err
}
