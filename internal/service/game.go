package service

import (
	"crypto"
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/hex"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
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
	Join(req request.GameJoinRequest) (err error)
	AllowJoin(req request.GameAllowJoinRequest) (err error)
}

type GameService struct {
	gameRepository       repository.IGameRepository
	gameTeamRepository   repository.IGameTeamRepository
	submissionRepository repository.ISubmissionRepository
	teamRepository       repository.ITeamRepository
}

func NewGameService(appRepository *repository.Repository) IGameService {
	return &GameService{
		gameRepository:       appRepository.GameRepository,
		gameTeamRepository:   appRepository.GameTeamRepository,
		submissionRepository: appRepository.SubmissionRepository,
		teamRepository:       appRepository.TeamRepository,
	}
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
	}
	return
}

func (g *GameService) Join(req request.GameJoinRequest) (err error) {
	games, _, err := g.gameRepository.Find(request.GameFindRequest{
		ID: req.ID,
	})
	game := games[0]
	teams, _, err := g.teamRepository.Find(request.TeamFindRequest{
		ID: req.TeamID,
	})
	team := teams[0]
	if req.UserID != team.Captain.ID {
		return
	}

	allowed := convertor.FalseP()

	if game.Password != "" {
		hasher := crypto.SHA256.New()
		hasher.Write([]byte(req.Password))
		hashBytes := hasher.Sum(nil)
		if hex.EncodeToString(hashBytes) != game.Password {
			return
		}
		allowed = convertor.TrueP()
	}

	gameTeam := model.GameTeam{
		TeamID:  team.ID,
		GameID:  game.ID,
		Allowed: allowed,
	}

	sig, _ := signature.Sign(game.PrivateKey, strconv.Itoa(int(team.ID)))
	gameTeam.Signature = fmt.Sprintf("%s:%s", strconv.Itoa(int(team.ID)), sig)

	err = g.gameTeamRepository.Insert(gameTeam)
	return err
}

func (g *GameService) AllowJoin(req request.GameAllowJoinRequest) (err error) {
	gameTeams, err := g.gameTeamRepository.Find(model.GameTeam{
		ID:     req.ID,
		TeamID: req.TeamID,
	})
	gameTeam := gameTeams[0]
	gameTeam.Allowed = req.Allowed
	err = g.gameTeamRepository.Update(gameTeam)
	return err
}

func (g *GameService) Create(req request.GameCreateRequest) (err error) {
	publicKey, privateKey, err := ed25519.GenerateKey(rand.Reader)
	game := model.Game{
		PublicKey:  base64.StdEncoding.EncodeToString(publicKey),
		PrivateKey: base64.StdEncoding.EncodeToString(privateKey),
	}
	err = mapstructure.Decode(req, &game)
	if req.Password != "" {
		hasher := crypto.SHA256.New()
		hasher.Write([]byte(req.Password))
		hashBytes := hasher.Sum(nil)
		game.Password = hex.EncodeToString(hashBytes)
	}
	_, err = g.gameRepository.Insert(game)
	return err
}

func (g *GameService) Update(req request.GameUpdateRequest) (err error) {
	game := model.Game{}
	err = mapstructure.Decode(req, &game)
	if req.Password != "" {
		hasher := crypto.SHA256.New()
		hasher.Write([]byte(req.Password))
		hashBytes := hasher.Sum(nil)
		game.Password = hex.EncodeToString(hashBytes)
	}
	err = g.gameRepository.Update(game)
	return err
}

func (g *GameService) Delete(req request.GameDeleteRequest) (err error) {
	return g.gameRepository.Delete(req)
}
