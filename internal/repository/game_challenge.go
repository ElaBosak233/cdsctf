package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type IGameChallengeRepository interface {
	Find(req request.GameChallengeFindRequest) (gameChallenges []model.GameChallenge, err error)
	Create(gameChallenge model.GameChallenge) (err error)
	Update(gameChallenge model.GameChallenge) (err error)
	Delete(gameChallenge model.GameChallenge) (err error)
}

type GameChallengeRepository struct {
	db *gorm.DB
}

func NewGameChallengeRepository(db *gorm.DB) IGameChallengeRepository {
	return &GameChallengeRepository{db: db}
}

func (t *GameChallengeRepository) Find(req request.GameChallengeFindRequest) (gameChallenges []model.GameChallenge, err error) {
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.GameID != 0 {
			q = q.Where("game_id = ?", req.GameID)
		}
		if req.ChallengeID != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeID)
		}
		if req.IsEnabled != nil {
			q = q.Where("is_enabled = ?", *(req.IsEnabled))
		}
		return q
	}
	db := applyFilters(t.db.Table("game_challenges"))
	result := db.
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
						Preload("Team", func(db *gorm.DB) *gorm.DB {
							return db.Select([]string{"id", "name", "email"})
						}).
						Order("submissions.created_at ASC").
						Where("status = ?", 2).
						Where("game_id = ?", req.GameID).
						Omit("flag")
				}).
				Omit("flags", "images", "is_practicable", "practice_pts", "created_at", "updated_at")
		}).
		Preload("Game").
		Find(&gameChallenges)
	return gameChallenges, result.Error
}

func (t *GameChallengeRepository) Create(gameChallenge model.GameChallenge) (err error) {
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
