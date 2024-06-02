package service

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"github.com/mitchellh/mapstructure"
	"strconv"
)

type IGameTeamService interface {
	Find(req request.GameTeamFindRequest) (teams []model.GameTeam, total int64, err error)
	Create(req request.GameTeamCreateRequest) (err error)
	Update(req request.GameTeamUpdateRequest) (err error)
	Delete(req request.GameTeamDeleteRequest) (err error)
}

type GameTeamService struct {
	gameTeamRepository   repository.IGameTeamRepository
	gameRepository       repository.IGameRepository
	teamRepository       repository.ITeamRepository
	submissionRepository repository.ISubmissionRepository
	userRepository       repository.IUserRepository
}

func NewGameTeamService(appRepository *repository.Repository) IGameTeamService {
	return &GameTeamService{
		submissionRepository: appRepository.SubmissionRepository,
		gameTeamRepository:   appRepository.GameTeamRepository,
		gameRepository:       appRepository.GameRepository,
		teamRepository:       appRepository.TeamRepository,
		userRepository:       appRepository.UserRepository,
	}
}

func (g *GameTeamService) Find(req request.GameTeamFindRequest) (teams []model.GameTeam, total int64, err error) {
	gameTeams, total, err := g.gameTeamRepository.Find(model.GameTeam{
		GameID: req.GameID,
		TeamID: req.TeamID,
	})
	for index, gameTeam := range gameTeams {
		if req.TeamID != 0 && gameTeam.TeamID != req.TeamID {
			continue
		}
		gameTeams[index] = gameTeam
	}
	return gameTeams, total, err
}

func (g *GameTeamService) Create(req request.GameTeamCreateRequest) (err error) {
	games, _, err := g.gameRepository.Find(request.GameFindRequest{
		ID: req.ID,
	})
	game := games[0]
	teams, _, err := g.teamRepository.Find(request.TeamFindRequest{
		ID: req.TeamID,
	})
	team := teams[0]
	users, _, err := g.userRepository.Find(request.UserFindRequest{
		ID: req.UserID,
	})
	user := users[0]
	if req.UserID != team.Captain.ID && (user.Group != "admin") {
		return errors.New("invalid team captain")
	}

	if int64(len(team.Users)) < game.MemberLimitMin || int64(len(team.Users)) > game.MemberLimitMax {
		return errors.New("invalid team member count")
	}

	gameTeams, _, err := g.gameTeamRepository.Find(model.GameTeam{
		GameID: req.ID,
	})
	for _, gameTeam := range gameTeams {
		if gameTeam.TeamID == team.ID && gameTeam.GameID == game.ID {
			return errors.New("team already exists")
		}
		for _, u := range gameTeam.Team.Users {
			for _, tu := range team.Users {
				if tu.ID == u.ID {
					return errors.New("user already exists")
				}
			}
		}
	}

	var isAllowed bool
	if game.IsPublic != nil && *game.IsPublic {
		isAllowed = true
	} else {
		isAllowed = false
	}

	gameTeam := model.GameTeam{
		TeamID:    team.ID,
		GameID:    game.ID,
		IsAllowed: &isAllowed,
	}

	gameTeam.Signature = fmt.Sprintf("%s:%s", strconv.Itoa(int(team.ID)), utils.HyphenlessUUID())

	err = g.gameTeamRepository.Create(gameTeam)
	return err
}

func (g *GameTeamService) Update(req request.GameTeamUpdateRequest) (err error) {
	var gameTeam model.GameTeam
	err = mapstructure.Decode(req, &gameTeam)
	err = g.gameTeamRepository.Update(gameTeam)
	return err
}

func (g *GameTeamService) Delete(req request.GameTeamDeleteRequest) (err error) {
	var gameTeam model.GameTeam
	err = mapstructure.Decode(req, &gameTeam)
	err = g.gameTeamRepository.Delete(gameTeam)
	return err
}
