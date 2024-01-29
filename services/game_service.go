package services

import (
	"crypto"
	"encoding/hex"
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/mitchellh/mapstructure"
	"math"
	"time"
)

type GameService interface {
	Find(req request.GameFindRequest) (games []response.GameResponse, pageCount int64, total int64, err error)
	Create(req request.GameCreateRequest) (err error)
	Update(req request.GameUpdateRequest) (err error)
	Delete(id int64) (err error)
}

type GameServiceImpl struct {
	GameRepository repositories.GameRepository
}

func NewGameServiceImpl(appRepository *repositories.AppRepository) GameService {
	return &GameServiceImpl{
		GameRepository: appRepository.GameRepository,
	}
}

func (g *GameServiceImpl) Find(req request.GameFindRequest) (games []response.GameResponse, pageCount int64, total int64, err error) {
	games, count, err := g.GameRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return games, pageCount, count, err
}

func (g *GameServiceImpl) Create(req request.GameCreateRequest) (err error) {
	gameEntity := entity.Game{}
	err = mapstructure.Decode(req, &gameEntity)
	if req.StartedAtUnix != 0 {
		gameEntity.StartedAt = time.Unix(req.StartedAtUnix, 0)
	}
	if req.EndedAtUnix != 0 {
		gameEntity.EndedAt = time.Unix(req.EndedAtUnix, 0)
	}
	if req.Password != "" {
		hasher := crypto.SHA256.New()
		hasher.Write([]byte(req.Password))
		hashBytes := hasher.Sum(nil)
		gameEntity.Password = hex.EncodeToString(hashBytes)
	}
	_, err = g.GameRepository.Insert(gameEntity)
	return err
}

func (g *GameServiceImpl) Update(req request.GameUpdateRequest) (err error) {
	gameEntity := entity.Game{}
	err = mapstructure.Decode(req, &gameEntity)
	if req.StartedAtUnix != 0 {
		gameEntity.StartedAt = time.Unix(req.StartedAtUnix, 0)
	}
	if req.EndedAtUnix != 0 {
		gameEntity.EndedAt = time.Unix(req.EndedAtUnix, 0)
	}
	if req.Password != "" {
		hasher := crypto.SHA256.New()
		hasher.Write([]byte(req.Password))
		hashBytes := hasher.Sum(nil)
		gameEntity.Password = hex.EncodeToString(hashBytes)
	}
	err = g.GameRepository.Update(gameEntity)
	return err
}

func (g *GameServiceImpl) Delete(id int64) (err error) {
	//TODO implement me
	panic("implement me")
}
