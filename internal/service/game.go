package service

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type IGameService interface {
	// Find will find games with the given request, and return the games and total count.
	Find(req request.GameFindRequest) ([]model.Game, int64, error)

	// Create will create a new game with the given request.
	Create(req request.GameCreateRequest) error

	// Update will update the game with the given request.
	Update(req request.GameUpdateRequest) error

	// Delete will delete the game with the given request.
	Delete(req request.GameDeleteRequest) error
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

func NewGameService(r *repository.Repository) IGameService {
	return &GameService{
		gameRepository:          r.GameRepository,
		gameChallengeRepository: r.GameChallengeRepository,
		gameTeamRepository:      r.GameTeamRepository,
		submissionRepository:    r.SubmissionRepository,
		challengeRepository:     r.ChallengeRepository,
		teamRepository:          r.TeamRepository,
		userRepository:          r.UserRepository,
	}
}

func (g *GameService) Create(req request.GameCreateRequest) error {
	publicKey, privateKey, err := ed25519.GenerateKey(rand.Reader)
	game := model.Game{
		PublicKey:  base64.StdEncoding.EncodeToString(publicKey),
		PrivateKey: base64.StdEncoding.EncodeToString(privateKey),
	}
	err = mapstructure.Decode(req, &game)
	_, err = g.gameRepository.Create(game)
	return err
}

func (g *GameService) Update(req request.GameUpdateRequest) error {
	game := model.Game{}
	err := mapstructure.Decode(req, &game)
	err = g.gameRepository.Update(game)
	return err
}

func (g *GameService) Delete(req request.GameDeleteRequest) error {
	return g.gameRepository.Delete(model.Game{
		ID: req.ID,
	})
}

func (g *GameService) Find(req request.GameFindRequest) ([]model.Game, int64, error) {
	games, total, err := g.gameRepository.Find(req)
	return games, total, err
}
