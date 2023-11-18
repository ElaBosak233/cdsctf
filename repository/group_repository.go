package repository

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type GroupRepository interface {
	Insert(user model.Group)
	Update(user model.Group)
	Delete(id string)
	FindById(id string) (group model.Group, err error)
	FindAll() []model.Group
}
