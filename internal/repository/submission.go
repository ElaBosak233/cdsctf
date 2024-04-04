package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type ISubmissionRepository interface {
	Create(submission model.Submission) (err error)
	Delete(id uint) (err error)
	Find(req request.SubmissionFindRequest) (submissions []model.Submission, count int64, err error)
}

type SubmissionRepository struct {
	db *gorm.DB
}

func NewSubmissionRepository(db *gorm.DB) ISubmissionRepository {
	return &SubmissionRepository{db: db}
}

func (t *SubmissionRepository) Create(submission model.Submission) (err error) {
	result := t.db.Table("submissions").Create(&submission)
	return result.Error
}

func (t *SubmissionRepository) Delete(id uint) (err error) {
	result := t.db.Table("submissions").Delete(&model.Submission{ID: id})
	return result.Error
}

func (t *SubmissionRepository) Find(req request.SubmissionFindRequest) (submissions []model.Submission, count int64, err error) {
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.UserID != 0 && req.TeamID == nil && req.GameID == nil {
			q = q.Where("user_id = ?", req.UserID)
		}
		if req.ChallengeID != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeID)
		}
		if req.TeamID != nil {
			q = q.Where("team_id = ?", *(req.TeamID))
		}
		if req.GameID != nil {
			q = q.Where("game_id = ?", *(req.GameID))
		}
		if req.Status != 0 {
			q = q.Where("status = ?", req.Status)
		}
		return q
	}
	db := applyFilters(t.db.Table("submissions"))

	result := db.Model(&model.Submission{}).Count(&count)
	if req.SortKey != "" && req.SortOrder != "" {
		db = db.Order(req.SortKey + " " + req.SortOrder)
	} else {
		db = db.Order("submissions.id DESC")
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	db = db.Joins("INNER JOIN users ON submissions.user_id = users.id").
		Joins("INNER JOIN challenges ON submissions.challenge_id = challenges.id")

	result = db.
		Preload("User", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "username", "nickname", "email"})
		}).
		Preload("Challenge", func(db *gorm.DB) *gorm.DB {
			return db.
				Preload("Category").
				Select([]string{"id", "title", "category_id", "difficulty", "practice_pts"})
		}).
		Preload("GameChallenge").
		Preload("Team", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "name", "email"})
		}).
		Preload("Game", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "title", "bio", "first_blood_reward_ratio", "second_blood_reward_ratio", "third_blood_reward_ratio"})
		}).
		Find(&submissions)
	return submissions, count, result.Error
}
