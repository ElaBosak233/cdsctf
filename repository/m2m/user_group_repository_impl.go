package m2m

import (
	"github.com/elabosak233/pgshub/model/data/m2m"
	"xorm.io/xorm"
)

type UserGroupRepositoryImpl struct {
	Db *xorm.Engine
}

func NewUserGroupRepositoryImpl(Db *xorm.Engine) UserGroupRepository {
	return &UserGroupRepositoryImpl{Db: Db}
}

func (t *UserGroupRepositoryImpl) Insert(userGroup m2m.UserGroup) error {
	_, err := t.Db.Table("user_group").Insert(&userGroup)
	return err
}

func (t *UserGroupRepositoryImpl) Delete(userGroup m2m.UserGroup) error {
	_, err := t.Db.Table("user_group").Delete(&userGroup)
	return err
}

func (t *UserGroupRepositoryImpl) Exist(userGroup m2m.UserGroup) (bool, error) {
	r, err := t.Db.Table("user_group").Exist(&userGroup)
	return r, err
}

func (t *UserGroupRepositoryImpl) FindByUserId(userId string) (userGroups []m2m.UserGroup, err error) {
	var userGroup []m2m.UserGroup
	err = t.Db.Table("user_group").
		Join("INNER", "group", "user_group.group_id = group.id").
		Where("user_group.user_id = ?", userId).
		Find(&userGroup)
	if err != nil {
		return nil, err
	}
	return userGroup, err
}

func (t *UserGroupRepositoryImpl) FindByGroupId(groupId string) (userGroups []m2m.UserGroup, err error) {
	var groupUser []m2m.UserGroup
	err = t.Db.Table("user_group").
		Join("INNER", "user", "user_group.user_id = user.id").
		Where("user_group.group_id = ?", groupId).
		Find(&groupUser)
	if err != nil {
		return nil, err
	}
	return groupUser, err
}

func (t *UserGroupRepositoryImpl) FindAll() (userGroups []m2m.UserGroup, err error) {
	var userGroup []m2m.UserGroup
	err = t.Db.Find(&userGroup)
	if err != nil {
		return nil, err
	}
	return userGroup, err
}
