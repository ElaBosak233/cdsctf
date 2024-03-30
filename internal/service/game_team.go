package service

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils/calculate"
	"github.com/elabosak233/cloudsdale/internal/utils/signature"
	"github.com/mitchellh/mapstructure"
	"strconv"
)

type IGameTeamService interface {
	Find(req request.GameTeamFindRequest) (teams []response.GameTeamResponse, err error)
	FindByID(req request.GameTeamFindRequest) (team response.GameTeamResponse, err error)
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

func (g *GameTeamService) Find(req request.GameTeamFindRequest) (teams []response.GameTeamResponse, err error) {
	gameTeams, err := g.gameTeamRepository.Find(model.GameTeam{
		GameID: req.GameID,
	})
	submissions, _, err := g.submissionRepository.Find(request.SubmissionFindRequest{
		GameID: &req.GameID,
		Status: 2,
	})
	for i := range gameTeams {
		gameTeams[i].Rank = 1
		gameTeams[i].Pts = 0
		gameTeams[i].Solved = 0
		for _, submission := range submissions {
			if submission.TeamID != nil && *(submission.TeamID) == gameTeams[i].TeamID && submission.GameChallenge != nil {
				gameTeams[i].Pts += calculate.GameChallengePts(
					submission.GameChallenge.MaxPts,
					submission.GameChallenge.MinPts,
					submission.Challenge.Difficulty,
					int(submission.Rank-1),
					submission.Game.FirstBloodRewardRatio,
					submission.Game.SecondBloodRewardRatio,
					submission.Game.ThirdBloodRewardRatio,
				)
				gameTeams[i].Solved++
			}
		}
	}
	for i := range gameTeams {
		for j := range gameTeams {
			if gameTeams[i].Pts < gameTeams[j].Pts {
				gameTeams[i].Rank++
			}
		}
	}
	for _, gameTeam := range gameTeams {
		if req.TeamID != 0 && gameTeam.TeamID != req.TeamID {
			continue
		}
		var team response.GameTeamResponse
		_ = mapstructure.Decode(gameTeam, &team)
		_ = mapstructure.Decode(*(gameTeam.Team), &team.Team)
		teams = append(teams, team)
	}
	return teams, err
}

func (g *GameTeamService) FindByID(req request.GameTeamFindRequest) (team response.GameTeamResponse, err error) {
	teams, err := g.Find(request.GameTeamFindRequest{
		TeamID: req.TeamID,
		GameID: req.GameID,
	})
	if len(teams) > 0 {
		team = teams[0]
	}
	return team, err
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
	if req.UserID != team.Captain.ID && (user.Group.Name != "admin") {
		return errors.New("invalid team captain")
	}

	if int64(len(team.Users)) < game.MemberLimitMin || int64(len(team.Users)) > game.MemberLimitMax {
		return errors.New("invalid team member count")
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

	sig, _ := signature.Sign(game.PrivateKey, strconv.Itoa(int(team.ID)))
	gameTeam.Signature = fmt.Sprintf("%s:%s", strconv.Itoa(int(team.ID)), sig)

	err = g.gameTeamRepository.Insert(gameTeam)
	return err
}

func (g *GameTeamService) Update(req request.GameTeamUpdateRequest) (err error) {
	gameTeams, err := g.gameTeamRepository.Find(model.GameTeam{
		GameID: req.ID,
		TeamID: req.TeamID,
	})
	gameTeam := gameTeams[0]
	gameTeam.IsAllowed = req.IsAllowed
	err = g.gameTeamRepository.Update(gameTeam)
	return err
}

func (g *GameTeamService) Delete(req request.GameTeamDeleteRequest) (err error) {
	gameTeams, err := g.gameTeamRepository.Find(model.GameTeam{
		GameID: req.GameID,
		TeamID: req.TeamID,
	})
	gameTeam := gameTeams[0]
	err = g.gameTeamRepository.Delete(gameTeam)
	return err
}
