package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IGameChallengeRepository interface {
	BatchFindByGameIdAndChallengeId(gameID uint, challengeIDs []uint) (gameChallenges []model.GameChallenge, err error)
}

type GameChallengeRepository struct {
	Db *gorm.DB
}

func NewGameChallengeRepository(Db *gorm.DB) IGameChallengeRepository {
	return &GameChallengeRepository{Db: Db}
}

func (t *GameChallengeRepository) BatchFindByGameIdAndChallengeId(gameID uint, challengeIDs []uint) (gameChallenges []model.GameChallenge, err error) {
	result := t.Db.Table("game_challenges").
		Where("game_id = ?", gameID).
		Where("challenge_id IN ?", challengeIDs).
		Find(&gameChallenges)
	return gameChallenges, result.Error
}
