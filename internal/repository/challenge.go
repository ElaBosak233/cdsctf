package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type IChallengeRepository interface {
	Create(challenge model.Challenge) (c model.Challenge, err error)
	Update(challenge model.Challenge) (c model.Challenge, err error)
	Delete(id uint) (err error)
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error)
}

type ChallengeRepository struct {
	db *gorm.DB
}

func NewChallengeRepository(db *gorm.DB) IChallengeRepository {
	return &ChallengeRepository{db: db}
}

func (t *ChallengeRepository) Create(challenge model.Challenge) (c model.Challenge, err error) {
	result := t.db.Table("challenges").Create(&challenge)
	return challenge, result.Error
}

func (t *ChallengeRepository) Delete(id uint) (err error) {
	result := t.db.Table("challenges").Delete(&model.Challenge{ID: id})
	return result.Error
}

func (t *ChallengeRepository) Update(challenge model.Challenge) (c model.Challenge, err error) {
	result := t.db.Table("challenges").Model(&challenge).Updates(&challenge)
	return challenge, result.Error
}

func (t *ChallengeRepository) Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error) {
	applyFilter := func(q *gorm.DB) *gorm.DB {
		if req.CategoryID != nil {
			q = q.Where("category_id = ?", *(req.CategoryID))
		}
		if req.Title != "" {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsPracticable != nil {
			q = q.Where("is_practicable = ?", *(req.IsPracticable))
		}
		if req.IsDynamic != nil {
			q = q.Where("is_dynamic = ?", *(req.IsDynamic))
		}
		if req.Difficulty > 0 {
			q = q.Where("difficulty = ?", req.Difficulty)
		}
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		return q
	}
	db := applyFilter(t.db.Table("challenges"))

	result := db.Model(&model.Challenge{}).Count(&count)
	if req.SortOrder != "" && req.SortKey != "" {
		db = db.Order(req.SortKey + " " + req.SortOrder)
	} else {
		db = db.Order("challenges.id ASC")
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	result = db.
		Preload("Category", func(db *gorm.DB) *gorm.DB {
			return db.Omit("created_at", "updated_at")
		}).
		Preload("Flags").
		Preload("Ports").
		Preload("Envs").
		Preload("Submissions", func(db *gorm.DB) *gorm.DB {
			return db.
				Preload("User", func(db *gorm.DB) *gorm.DB {
					return db.Select([]string{"id", "username", "nickname", "email"})
				}).
				Preload("Team").
				Preload("Game").
				Order("submissions.created_at ASC").
				Where("submissions.status = ?", 2).
				Omit("flag")
		}).
		Preload("Solved", func(db *gorm.DB) *gorm.DB {
			return db.
				Where("status = ?", 2).
				Where("user_id = ?", req.UserID).
				Omit("flag")
		}).
		Find(&challenges)
	return challenges, count, result.Error
}
