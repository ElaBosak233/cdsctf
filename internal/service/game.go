package service

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils/calculate"
	"github.com/elabosak233/cloudsdale/internal/utils/signature"
	"github.com/mitchellh/mapstructure"
	"math"
	"strconv"
)

type IGameService interface {
	Find(req request.GameFindRequest) (games []response.GameResponse, pageCount int64, total int64, err error)
	Create(req request.GameCreateRequest) (err error)
	Update(req request.GameUpdateRequest) (err error)
	Delete(req request.GameDeleteRequest) (err error)
	Scoreboard(id uint) (submissions []model.Submission, err error)
	FindChallenge(req request.GameChallengeFindRequest) (challenges []response.GameChallengeResponse, err error)
	CreateChallenge(req request.GameChallengeCreateRequest) (err error)
	UpdateChallenge(req request.GameChallengeUpdateRequest) (err error)
	DeleteChallenge(req request.GameChallengeDeleteRequest) (err error)
	FindTeam(req request.GameTeamFindRequest) (teams []response.GameTeamResponse, err error)
	CreateTeam(req request.GameTeamCreateRequest) (err error)
	UpdateTeam(req request.GameTeamUpdateRequest) (err error)
	DeleteTeam(req request.GameTeamDeleteRequest) (err error)
}

type GameService struct {
	gameRepository          repository.IGameRepository
	gameChallengeRepository repository.IGameChallengeRepository
	gameTeamRepository      repository.IGameTeamRepository
	submissionRepository    repository.ISubmissionRepository
	challengeRepository     repository.IChallengeRepository
	teamRepository          repository.ITeamRepository
	userRepository          repository.IUserRepository
}

func NewGameService(appRepository *repository.Repository) IGameService {
	return &GameService{
		gameRepository:          appRepository.GameRepository,
		gameChallengeRepository: appRepository.GameChallengeRepository,
		gameTeamRepository:      appRepository.GameTeamRepository,
		submissionRepository:    appRepository.SubmissionRepository,
		challengeRepository:     appRepository.ChallengeRepository,
		teamRepository:          appRepository.TeamRepository,
		userRepository:          appRepository.UserRepository,
	}
}

func (g *GameService) Create(req request.GameCreateRequest) (err error) {
	publicKey, privateKey, err := ed25519.GenerateKey(rand.Reader)
	game := model.Game{
		PublicKey:  base64.StdEncoding.EncodeToString(publicKey),
		PrivateKey: base64.StdEncoding.EncodeToString(privateKey),
	}
	err = mapstructure.Decode(req, &game)
	_, err = g.gameRepository.Insert(game)
	return err
}

func (g *GameService) Update(req request.GameUpdateRequest) (err error) {
	game := model.Game{}
	err = mapstructure.Decode(req, &game)
	err = g.gameRepository.Update(game)
	return err
}

func (g *GameService) Delete(req request.GameDeleteRequest) (err error) {
	return g.gameRepository.Delete(req)
}

func (g *GameService) Find(req request.GameFindRequest) (games []response.GameResponse, pageCount int64, total int64, err error) {
	games, count, err := g.gameRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return games, pageCount, count, err
}

func (g *GameService) Scoreboard(id uint) (submissions []model.Submission, err error) {
	submissions, _, err = g.submissionRepository.Find(request.SubmissionFindRequest{
		GameID: &id,
		Status: 2,
	})
	for i := range submissions {
		submissions[i].Flag = ""
		submissions[i].Game = nil
	}
	return
}

func (g *GameService) FindChallenge(req request.GameChallengeFindRequest) (challenges []response.GameChallengeResponse, err error) {
	games, _, _ := g.gameRepository.Find(request.GameFindRequest{
		ID: req.GameID,
	})
	game := games[0]
	gameChallenges, err := g.gameChallengeRepository.Find(req)
	for _, gameChallenge := range gameChallenges {
		var challenge response.GameChallengeResponse
		_ = mapstructure.Decode(gameChallenge, &challenge)
		ss := challenge.MaxPts
		R := challenge.MinPts
		d := challenge.Difficulty
		x := len(challenge.Submissions)
		pts := calculate.ChallengePts(int64(ss), int64(R), d, x)
		switch x {
		case 0:
			pts = int64(math.Floor(((game.FirstBloodRewardRatio / 100) + 1) * float64(pts)))
		case 1:
			pts = int64(math.Floor(((game.SecondBloodRewardRatio / 100) + 1) * float64(pts)))
		case 2:
			pts = int64(math.Floor(((game.ThirdBloodRewardRatio / 100) + 1) * float64(pts)))
		}
		challenge.Pts = pts
		for _, submission := range challenge.Submissions {
			if req.TeamID != 0 && submission.TeamID == req.TeamID {
				sub := submission
				challenge.Solved = sub
				break
			}
		}
		switch x {
		case 0:
			challenge.Submissions = challenge.Submissions[:0]
		case 1:
			challenge.Submissions = challenge.Submissions[:1]
		case 2:
		default:
			challenge.Submissions = challenge.Submissions[:2]
		}
		challenges = append(challenges, challenge)
	}
	return challenges, err
}

func (g *GameService) CreateChallenge(req request.GameChallengeCreateRequest) (err error) {
	var gameChallenge model.GameChallenge
	err = mapstructure.Decode(req, &gameChallenge)
	err = g.gameChallengeRepository.Insert(gameChallenge)
	return err
}

func (g *GameService) UpdateChallenge(req request.GameChallengeUpdateRequest) (err error) {
	var gameChallenge model.GameChallenge
	err = mapstructure.Decode(req, &gameChallenge)
	err = g.gameChallengeRepository.Update(gameChallenge)
	return err
}

func (g *GameService) DeleteChallenge(req request.GameChallengeDeleteRequest) (err error) {
	var gameChallenge model.GameChallenge
	err = mapstructure.Decode(req, &gameChallenge)
	err = g.gameChallengeRepository.Delete(gameChallenge)
	return err
}

func (g *GameService) FindTeam(req request.GameTeamFindRequest) (teams []response.GameTeamResponse, err error) {
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
			if submission.TeamID == gameTeams[i].TeamID {
				gameTeams[i].Pts += submission.Pts
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

func (g *GameService) CreateTeam(req request.GameTeamCreateRequest) (err error) {
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
	if req.UserID != team.Captain.ID && (user.Group.Name != "admin" && user.Group.Name != "monitor") {
		return errors.New("invalid team captain")
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

func (g *GameService) UpdateTeam(req request.GameTeamUpdateRequest) (err error) {
	gameTeams, err := g.gameTeamRepository.Find(model.GameTeam{
		GameID: req.ID,
		TeamID: req.TeamID,
	})
	gameTeam := gameTeams[0]
	gameTeam.IsAllowed = req.IsAllowed
	err = g.gameTeamRepository.Update(gameTeam)
	return err
}

func (g *GameService) DeleteTeam(req request.GameTeamDeleteRequest) (err error) {
	gameTeams, err := g.gameTeamRepository.Find(model.GameTeam{
		GameID: req.GameID,
		TeamID: req.TeamID,
	})
	gameTeam := gameTeams[0]
	err = g.gameTeamRepository.Delete(gameTeam)
	return err
}
