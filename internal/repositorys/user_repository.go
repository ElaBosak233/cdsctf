package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
)

type UserRepository interface {
	Insert(user model.User) error
	Update(user model.User) error
	Delete(id string) error
	FindById(id string) (user model.User, err error)
	FindByUsername(username string) (user model.User, err error)
	FindAll() ([]model.User, error)
}
