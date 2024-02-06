package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type PortRepository interface {
	Insert(port entity.Port) (p entity.Port, err error)
	InsertMulti(ports []entity.Port) (err error)
	Update(port entity.Port) (p entity.Port, err error)
	Delete(port entity.Port) (err error)
	FindByImageID(imageIDs []int64) (ports []entity.Port, err error)
	DeleteByImageID(imageIDs []int64) (err error)
}

type PortRepositoryImpl struct {
	Db *xorm.Engine
}

func NewPortRepositoryImpl(Db *xorm.Engine) PortRepository {
	return &PortRepositoryImpl{Db: Db}
}

func (t *PortRepositoryImpl) Insert(port entity.Port) (p entity.Port, err error) {
	_, err = t.Db.Table("port").Insert(&port)
	return port, err
}

func (t *PortRepositoryImpl) InsertMulti(ports []entity.Port) (err error) {
	_, err = t.Db.Table("port").Insert(&ports)
	return err
}

func (t *PortRepositoryImpl) Update(port entity.Port) (p entity.Port, err error) {
	_, err = t.Db.Table("port").ID(port.PortID).Update(&port)
	return port, err
}

func (t *PortRepositoryImpl) Delete(port entity.Port) (err error) {
	_, err = t.Db.Table("port").ID(port.PortID).Delete(&port)
	return err
}

func (t *PortRepositoryImpl) FindByImageID(imageIDs []int64) (ports []entity.Port, err error) {
	err = t.Db.Table("port").In("image_id", imageIDs).Find(&ports)
	return ports, err
}

func (t *PortRepositoryImpl) DeleteByImageID(imageIDs []int64) (err error) {
	_, err = t.Db.Table("port").In("image_id", imageIDs).Delete(&entity.Port{})
	return err
}
