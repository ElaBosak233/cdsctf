package repository

import (
	"github.com/elabosak233/pgshub/internal/model/data"
)

type UserRepository interface {
	Save(user data.User)
	Update(user data.User)
	Delete(id string)
	FindById(id string) (user data.User, err error)
	FindByUsername(username string) (user data.User, err error)
	FindAll() []data.User
}
