package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IGameChallengeRepository interface {
	BatchFindByGameIdAndChallengeId(gameID uint, challengeIDs []uint) (gameChallenges []model.GameChallenge, err error)
}

type GameChallengeRepository struct {
	db *gorm.DB
}

func NewGameChallengeRepository(db *gorm.DB) IGameChallengeRepository {
	return &GameChallengeRepository{db: db}
}

func (t *GameChallengeRepository) BatchFindByGameIdAndChallengeId(gameID uint, challengeIDs []uint) (gameChallenges []model.GameChallenge, err error) {
	result := t.db.Table("game_challenges").
		Where("game_id = ?", gameID).
		Where("challenge_id IN ?", challengeIDs).
		Find(&gameChallenges)
	return gameChallenges, result.Error
}
