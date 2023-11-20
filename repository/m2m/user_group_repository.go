package m2m

import model "github.com/elabosak233/pgshub/model/data/m2m"

type UserGroupRepository interface {
	Insert(userGroup model.UserGroup) error
	Delete(userGroup model.UserGroup) error
	Exist(userGroup model.UserGroup) (bool, error)
	FindByUserId(userId string) (userGroups []model.UserGroup, err error)
	FindByGroupId(groupId string) (userGroups []model.UserGroup, err error)
	FindAll() (userGroups []model.UserGroup, err error)
}
