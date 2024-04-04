package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IHintRepository interface {
	Create(hint model.Hint) (h model.Hint, err error)
	Update(hint model.Hint) (h model.Hint, err error)
	Delete(hint model.Hint) (err error)
}

type HintRepository struct {
	db *gorm.DB
}

func NewHintRepository(db *gorm.DB) IHintRepository {
	return &HintRepository{db: db}
}

func (t *HintRepository) Create(hint model.Hint) (h model.Hint, err error) {
	result := t.db.Table("hints").Create(&hint)
	return hint, result.Error
}

func (t *HintRepository) Update(hint model.Hint) (h model.Hint, err error) {
	result := t.db.Table("hints").Model(&hint).Updates(&hint)
	return hint, result.Error
}

func (t *HintRepository) Delete(hint model.Hint) (err error) {
	result := t.db.Table("hints").Delete(&hint)
	return result.Error
}
