package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type ISubmissionRepository interface {
	Insert(submission model.Submission) (err error)
	Delete(id uint) (err error)
	Find(req request.SubmissionFindRequest) (submissions []model.Submission, count int64, err error)
	FindByChallengeID(req request.SubmissionFindByChallengeIDRequest) (submissions []model.Submission, err error)
}

type SubmissionRepository struct {
	db *gorm.DB
}

func NewSubmissionRepository(db *gorm.DB) ISubmissionRepository {
	return &SubmissionRepository{db: db}
}

func (t *SubmissionRepository) Insert(submission model.Submission) (err error) {
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
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Order("submissions." + sortKey + " ASC")
		} else if sortOrder == "desc" {
			db = db.Order("submissions." + sortKey + " DESC")
		}
	} else {
		db = db.Order("submissions.id DESC") // 默认采用 IDs 降序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	db = db.Joins("INNER JOIN users ON submissions.user_id = users.id").
		Joins("INNER JOIN challenges ON submissions.challenge_id = challenges.id").
		Joins("LEFT JOIN teams ON submissions.team_id = teams.id").
		Joins("LEFT JOIN games ON submissions.game_id = games.id")
	result = db.
		Preload("User", func(Db *gorm.DB) *gorm.DB {
			return Db.Select([]string{"id", "username", "nickname", "email"})
		}).
		Preload("Challenge").
		Preload("Team").
		Preload("Game").
		Find(&submissions)
	return submissions, count, result.Error
}

func (t *SubmissionRepository) FindByChallengeID(req request.SubmissionFindByChallengeIDRequest) (submissions []model.Submission, err error) {
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.UserID != 0 {
			q = q.Where("submissions.user_id = ?", req.UserID)
		}
		if req.TeamID != nil {
			q = q.Where("submissions.team_id = ?", *(req.TeamID))
		}
		if req.GameID != nil {
			q = q.Where("submissions.game_id = ?", *(req.GameID))
		}
		if req.Status != 0 {
			q = q.Where("submissions.status = ?", req.Status)
		}
		return q
	}
	db := applyFilters(t.db.Table("submissions"))
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Order("submissions." + sortKey + " ASC")
		} else if sortOrder == "desc" {
			db = db.Order("submissions." + sortKey + " DESC")
		}
	}
	db = db.Joins("INNER JOIN users ON submissions.user_id = users.id").
		Joins("LEFT JOIN teams ON submissions.team_id = teams.id").
		Joins("LEFT JOIN challenges ON submissions.challenge_id = challenges.id").
		Where("submissions.challenge_id IN ?", req.ChallengeID)
	_ = db.Find(&submissions)
	return submissions, err
}
