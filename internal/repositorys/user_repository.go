package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
)

type UserRepository interface {
	Insert(user model.User) error
	Update(user model.User) error
	Delete(id string) error
	FindById(id string) (user model.User, err error)
	FindByUsername(username string) (user model.User, err error)
	FindByEmail(email string) (user model.User, err error)
	Find(req request.UserFindRequest) (user []model.User, count int64, err error)
}
