package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type IGameChallengeRepository interface {
	FindByGameID(req request.GameChallengeFindRequest) (gameChallenges []model.GameChallenge, err error)
	Insert(gameChallenge model.GameChallenge) (err error)
	Update(gameChallenge model.GameChallenge) (err error)
	Delete(gameChallenge model.GameChallenge) (err error)
	BatchFindByGameIdAndChallengeId(gameID uint, challengeIDs []uint) (gameChallenges []model.GameChallenge, err error)
}

type GameChallengeRepository struct {
	db *gorm.DB
}

func NewGameChallengeRepository(db *gorm.DB) IGameChallengeRepository {
	return &GameChallengeRepository{db: db}
}

func (t *GameChallengeRepository) FindByGameID(req request.GameChallengeFindRequest) (gameChallenges []model.GameChallenge, err error) {
	result := t.db.Table("game_challenges").
		Where("game_id = ?", req.GameID).
		Preload("Challenge", func(db *gorm.DB) *gorm.DB {
			return db.
				Preload("Category", func(dv *gorm.DB) *gorm.DB {
					return dv.Omit("created_at", "updated_at")
				}).
				Preload("Submissions", func(db *gorm.DB) *gorm.DB {
					return db.
						Preload("User", func(db *gorm.DB) *gorm.DB {
							return db.Select([]string{"id", "username", "nickname", "email"})
						}).
						Preload("Team").
						Order("submissions.created_at ASC").
						Where("status = ?", 2).
						Omit("flag")
				}).
				Preload("Solved", func(db *gorm.DB) *gorm.DB {
					return db.
						Where("status = ?", 2).
						Where("team_id = ?", req.TeamID).
						Limit(1).
						Omit("flag")
				}).
				Omit("flags", "images", "is_practicable", "practice_pts", "created_at", "updated_at")
		}).
		Find(&gameChallenges)
	return gameChallenges, result.Error
}

func (t *GameChallengeRepository) BatchFindByGameIdAndChallengeId(gameID uint, challengeIDs []uint) (gameChallenges []model.GameChallenge, err error) {
	result := t.db.Table("game_challenges").
		Where("game_id = ?", gameID).
		Where("challenge_id IN ?", challengeIDs).
		Find(&gameChallenges)
	return gameChallenges, result.Error
}

func (t *GameChallengeRepository) Insert(gameChallenge model.GameChallenge) (err error) {
	result := t.db.Table("game_challenges").Create(&gameChallenge)
	return result.Error
}

func (t *GameChallengeRepository) Update(gameChallenge model.GameChallenge) (err error) {
	result := t.db.Table("game_challenges").
		Where("challenge_id = ?", gameChallenge.ChallengeID).
		Where("game_id = ?", gameChallenge.GameID).
		Updates(&gameChallenge)
	return result.Error
}

func (t *GameChallengeRepository) Delete(gameChallenge model.GameChallenge) (err error) {
	result := t.db.Table("game_challenges").
		Where("game_id = ?", gameChallenge.GameID).
		Where("challenge_id = ?", gameChallenge.ChallengeID).
		Delete(&gameChallenge)
	return result.Error
}
