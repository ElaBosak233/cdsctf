package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IPortRepository interface {
	Insert(port model.Port) (p model.Port, err error)
	Update(port model.Port) (p model.Port, err error)
	Delete(port model.Port) (err error)
	FindByImageID(imageIDs []uint) (ports []model.Port, err error)
	DeleteByImageID(imageIDs []uint) (err error)
}

type PortRepository struct {
	Db *gorm.DB
}

func NewPortRepository(Db *gorm.DB) IPortRepository {
	return &PortRepository{Db: Db}
}

func (t *PortRepository) Insert(port model.Port) (p model.Port, err error) {
	result := t.Db.Table("ports").Create(&port)
	return port, result.Error
}

func (t *PortRepository) Update(port model.Port) (p model.Port, err error) {
	result := t.Db.Table("ports").Model(&port).Updates(&port)
	return port, result.Error
}

func (t *PortRepository) Delete(port model.Port) (err error) {
	result := t.Db.Table("ports").Delete(&port)
	return result.Error
}

func (t *PortRepository) FindByImageID(imageIDs []uint) (ports []model.Port, err error) {
	result := t.Db.Table("ports").Where("image_id IN ?", imageIDs).Find(&ports)
	return ports, result.Error
}

func (t *PortRepository) DeleteByImageID(imageIDs []uint) (err error) {
	result := t.Db.Table("ports").Where("image_id IN ?", imageIDs).Delete(&model.Port{})
	return result.Error
}
