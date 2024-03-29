package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type INoticeRepository interface {
	Insert(notice model.Notice) (n model.Notice, err error)
	Update(notice model.Notice) (n model.Notice, err error)
	Delete(notice model.Notice) (err error)
}

type NoticeRepository struct {
	db *gorm.DB
}

func NewNoticeRepository(db *gorm.DB) INoticeRepository {
	return &NoticeRepository{db: db}
}

func (t *NoticeRepository) Insert(notice model.Notice) (n model.Notice, err error) {
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
