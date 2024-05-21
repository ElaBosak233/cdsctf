package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type INoticeRepository interface {
	Find(req request.NoticeFindRequest) (notices []model.Notice, total int64, err error)
	Create(notice model.Notice) (n model.Notice, err error)
	Update(notice model.Notice) (n model.Notice, err error)
	Delete(notice model.Notice) (err error)
}

type NoticeRepository struct {
	db *gorm.DB
}

func NewNoticeRepository(db *gorm.DB) INoticeRepository {
	return &NoticeRepository{db: db}
}

func (t *NoticeRepository) Find(req request.NoticeFindRequest) (notices []model.Notice, total int64, err error) {
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.GameID != 0 {
			q = q.Where("game_id = ?", req.GameID)
		}
		if req.Type != "" {
			q = q.Where("type = ?", req.Type)
		}
		return q
	}
	db := applyFilters(t.db.Table("notices"))
	result := db.Model(&model.Notice{}).Count(&total)
	db = db.Order("notices.id DESC")
	result = db.
		Preload("User", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "username", "nickname", "email"})
		}).
		Preload("Team", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "name", "email"})
		}).
		Preload("Challenge", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "title"})
		}).
		Find(&notices)
	return notices, total, result.Error
}

func (t *NoticeRepository) Create(notice model.Notice) (n model.Notice, err error) {
	result := t.db.Table("notices").Create(&notice)
	return notice, result.Error
}

func (t *NoticeRepository) Update(notice model.Notice) (n model.Notice, err error) {
	result := t.db.Table("notices").Save(&notice)
	return notice, result.Error
}

func (t *NoticeRepository) Delete(notice model.Notice) (err error) {
	result := t.db.Table("notices").Delete(&notice)
	return result.Error
}
