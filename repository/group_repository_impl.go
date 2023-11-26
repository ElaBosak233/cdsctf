package repository

import (
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/utils"
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
	var group model.Group
	_, err := t.Db.Table("`group`").Where("id = ?", id).Delete(&group)
	utils.ErrorPanic(err)
}

// FindAll implements UserRepository
func (t *GroupRepositoryImpl) FindAll() []model.Group {
	var group []model.Group
	err := t.Db.Find(&group)
	utils.ErrorPanic(err)
	return group
}

// FindById implements UserRepository
func (t *GroupRepositoryImpl) FindById(id string) (model.Group, error) {
	var group model.Group
	_, err := t.Db.Table("`group`").Where("id = ?", id).Get(&group)
	utils.ErrorPanic(err)
	return group, nil
}

// Insert implements UserRepository
func (t *GroupRepositoryImpl) Insert(group model.Group) {
	_, err := t.Db.Insert(&group)
	utils.ErrorPanic(err)
}

// Update implements UserRepository
func (t *GroupRepositoryImpl) Update(group model.Group) {
	_, err := t.Db.Table("`group`").ID(group.GroupId).Update(&group)
	utils.ErrorPanic(err)
}
