package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IGameChallengeRepository interface {
	BatchFindByGameIdAndChallengeId(gameID int64, challengeIDs []int64) (gameChallenges []model.GameChallenge, err error)
}

type GameChallengeRepository struct {
	Db *xorm.Engine
}

func NewGameChallengeRepository(Db *xorm.Engine) IGameChallengeRepository {
	return &GameChallengeRepository{Db: Db}
}

func (t *GameChallengeRepository) BatchFindByGameIdAndChallengeId(gameID int64, challengeIDs []int64) (gameChallenges []model.GameChallenge, err error) {
	err = t.Db.Table("game_challenge").Where("game_id = ?", gameID).In("challenge_id", challengeIDs).Find(&gameChallenges)
	return gameChallenges, err
}
