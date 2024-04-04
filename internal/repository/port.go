package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IPortRepository interface {
	Create(port model.Port) (p model.Port, err error)
	Update(port model.Port) (p model.Port, err error)
	Delete(port model.Port) (err error)
	FindByImageID(imageIDs []uint) (ports []model.Port, err error)
	DeleteByImageID(imageIDs []uint) (err error)
}

type PortRepository struct {
	db *gorm.DB
}

func NewPortRepository(db *gorm.DB) IPortRepository {
	return &PortRepository{db: db}
}

func (t *PortRepository) Create(port model.Port) (p model.Port, err error) {
	result := t.db.Table("ports").Create(&port)
	return port, result.Error
}

func (t *PortRepository) Update(port model.Port) (p model.Port, err error) {
	result := t.db.Table("ports").Model(&port).Updates(&port)
	return port, result.Error
}

func (t *PortRepository) Delete(port model.Port) (err error) {
	result := t.db.Table("ports").Delete(&port)
	return result.Error
}

func (t *PortRepository) FindByImageID(imageIDs []uint) (ports []model.Port, err error) {
	result := t.db.Table("ports").Where("image_id IN ?", imageIDs).Find(&ports)
	return ports, result.Error
}

func (t *PortRepository) DeleteByImageID(imageIDs []uint) (err error) {
	result := t.db.Table("ports").Where("image_id IN ?", imageIDs).Delete(&model.Port{})
	return result.Error
}
