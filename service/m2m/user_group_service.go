package m2m

import (
	model "github.com/elabosak233/pgshub/model/data/m2m"
)

type UserGroupService interface {
	Create(req model.UserGroup) error
	Delete(req model.UserGroup) error
	FindByUserId(userId string) ([]model.UserGroup, error)
	FindByGroupId(groupId string) ([]model.UserGroup, error)
	FindAll() ([]model.UserGroup, error)
}
