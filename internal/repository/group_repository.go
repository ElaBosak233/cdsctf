package repository

import (
	"github.com/elabosak233/pgshub/internal/model/data"
)

type GroupRepository interface {
	Insert(user data.Group)
	Update(user data.Group)
	Delete(id string)
	FindById(id string) (group data.Group, err error)
	FindAll() []data.Group
}
