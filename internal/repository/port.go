package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IPortRepository interface {
	Insert(port model.Port) (p model.Port, err error)
	InsertMulti(ports []model.Port) (err error)
	Update(port model.Port) (p model.Port, err error)
	Delete(port model.Port) (err error)
	FindByImageID(imageIDs []int64) (ports []model.Port, err error)
	DeleteByImageID(imageIDs []int64) (err error)
}

type PortRepository struct {
	Db *xorm.Engine
}

func NewPortRepository(Db *xorm.Engine) IPortRepository {
	return &PortRepository{Db: Db}
}

func (t *PortRepository) Insert(port model.Port) (p model.Port, err error) {
	_, err = t.Db.Table("port").Insert(&port)
	return port, err
}

func (t *PortRepository) InsertMulti(ports []model.Port) (err error) {
	_, err = t.Db.Table("port").Insert(&ports)
	return err
}

func (t *PortRepository) Update(port model.Port) (p model.Port, err error) {
	_, err = t.Db.Table("port").ID(port.ID).Update(&port)
	return port, err
}

func (t *PortRepository) Delete(port model.Port) (err error) {
	_, err = t.Db.Table("port").ID(port.ID).Delete(&port)
	return err
}

func (t *PortRepository) FindByImageID(imageIDs []int64) (ports []model.Port, err error) {
	err = t.Db.Table("port").In("image_id", imageIDs).Find(&ports)
	return ports, err
}

func (t *PortRepository) DeleteByImageID(imageIDs []int64) (err error) {
	_, err = t.Db.Table("port").In("image_id", imageIDs).Delete(&model.Port{})
	return err
}
