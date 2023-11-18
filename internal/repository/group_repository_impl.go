package repository

import (
	"github.com/elabosak233/pgshub/internal/model/data"
	"github.com/elabosak233/pgshub/internal/utils"
	"xorm.io/xorm"
)

type GroupRepositoryImpl struct {
	Db *xorm.Engine
}

func NewGroupRepositoryImpl(Db *xorm.Engine) GroupRepository {
	return &GroupRepositoryImpl{Db: Db}
}

// Delete implements UserRepository
func (t *GroupRepositoryImpl) Delete(id string) {
	var group data.Group
	_, err := t.Db.Table("`group`").Where("id = ?", id).Delete(&group)
	utils.ErrorPanic(err)
}

// FindAll implements UserRepository
func (t *GroupRepositoryImpl) FindAll() []data.Group {
	var group []data.Group
	err := t.Db.Find(&group)
	utils.ErrorPanic(err)
	return group
}

// FindById implements UserRepository
func (t *GroupRepositoryImpl) FindById(id string) (data.Group, error) {
	var group data.Group
	_, err := t.Db.Table("`group`").Where("id = ?", id).Get(&group)
	utils.ErrorPanic(err)
	return group, nil
}

// Insert implements UserRepository
func (t *GroupRepositoryImpl) Insert(group data.Group) {
	_, err := t.Db.Insert(&group)
	utils.ErrorPanic(err)
}

// Update implements UserRepository
func (t *GroupRepositoryImpl) Update(group data.Group) {
	_, err := t.Db.Table("`group`").ID(group.Id).Update(&group)
	utils.ErrorPanic(err)
}
