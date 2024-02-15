package service

import (
	"errors"
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	"github.com/elabosak233/pgshub/internal/repository"
	"math"
)

type ITeamService interface {
	Create(req request.TeamCreateRequest) error
	Update(req request.TeamUpdateRequest) error
	Delete(id uint) error
	Join(req request.TeamJoinRequest) (err error)
	Quit(req request.TeamQuitRequest) (err error)
	Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, total int64, err error)
	BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error)
	FindById(id uint) (res response.TeamResponse, err error)
}

type TeamService struct {
	UserRepository     repository.IUserRepository
	TeamRepository     repository.ITeamRepository
	UserTeamRepository repository.IUserTeamRepository
}

func NewTeamService(appRepository *repository.Repository) ITeamService {
	return &TeamService{
		UserRepository:     appRepository.UserRepository,
		TeamRepository:     appRepository.TeamRepository,
		UserTeamRepository: appRepository.UserTeamRepository,
	}
}

//// Mixin 用于向 Team 响应实体中混入 User 实体
//func (t *TeamService) Mixin(teams []response.TeamResponse) (ts []response.TeamResponse, err error) {
//	var teamIds []uint
//	teamsMap := make(map[uint]response.TeamResponse)
//	for _, team := range teams {
//		if _, ok := teamsMap[team.ID]; !ok {
//			teamsMap[team.ID] = team
//		}
//		teamIds = append(teamIds, team.ID)
//	}
//	users, err := t.UserRepository.BatchFindByTeamId(request.UserBatchFindByTeamIdRequest{
//		TeamID: teamIds,
//	})
//	for _, user := range users {
//		var userResponse response.UserResponse
//		_ = mapstructure.Decode(user, &userResponse)
//		if team, ok := teamsMap[user.Team]; ok {
//			if team.CaptainId == user.ID {
//				team.Captain = userResponse
//			}
//			team.Users = append(team.Users, userResponse)
//			teamsMap[user.TeamId] = team
//		}
//	}
//	for index, team := range teams {
//		teams[index].Users = teamsMap[team.ID].Users
//		teams[index].Captain = teamsMap[team.ID].Captain
//	}
//	return teams, err
//}

func (t *TeamService) Create(req request.TeamCreateRequest) error {
	user, err := t.UserRepository.FindById(req.CaptainId)
	if user.ID != 0 && err == nil {
		isLocked := false
		team, err := t.TeamRepository.Insert(model.Team{
			Name:        req.Name,
			CaptainID:   req.CaptainId,
			Description: req.Description,
			IsLocked:    &isLocked,
		})
		err = t.UserTeamRepository.Insert(model.UserTeam{
			TeamID: team.ID,
			UserID: req.CaptainId,
		})
		return err
	}
	return errors.New("用户不存在")
}

func (t *TeamService) Update(req request.TeamUpdateRequest) error {
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
			err = t.TeamRepository.Update(model.Team{
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
	team, err := t.TeamRepository.FindById(id)
	if team.ID != 0 {
		err = t.TeamRepository.Delete(id)
		err = t.UserTeamRepository.DeleteByTeamId(id)
		return err
	} else {
		return errors.New("团队不存在")
	}
}

func (t *TeamService) Find(req request.TeamFindRequest) (teams []response.TeamResponse, pageCount int64, total int64, err error) {
	teams, count, err := t.TeamRepository.Find(req)
	//teams, err = t.Mixin(teams)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return teams, pageCount, count, err
}

func (t *TeamService) BatchFind(req request.TeamBatchFindRequest) (teams []response.TeamResponse, err error) {
	teams, err = t.TeamRepository.BatchFind(req)
	//teams, err = t.Mixin(teams)
	return teams, err
}

func (t *TeamService) Join(req request.TeamJoinRequest) error {
	user, err := t.UserRepository.FindById(req.UserID)
	if user.ID != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamID)
		if team.ID != 0 {
			err = t.UserTeamRepository.Insert(model.UserTeam{
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
	user, err := t.UserRepository.FindById(req.UserID)
	if user.ID != 0 && err == nil {
		team, err := t.TeamRepository.FindById(req.TeamID)
		if team.ID != 0 {
			err = t.UserTeamRepository.Delete(model.UserTeam{
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
	teams, _, err := t.TeamRepository.Find(request.TeamFindRequest{
		ID: id,
	})
	if len(teams) > 0 {
		team = teams[0]
	}
	return team, err
}
