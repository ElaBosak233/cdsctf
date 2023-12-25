package implements

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/repositorys"
	"xorm.io/xorm"
)

type UserRepositoryImpl struct {
	Db *xorm.Engine
}

func NewUserRepositoryImpl(Db *xorm.Engine) repositorys.UserRepository {
	return &UserRepositoryImpl{Db: Db}
}

// Insert implements UserRepository
func (t *UserRepositoryImpl) Insert(user model.User) error {
	_, err := t.Db.Table("user").Insert(&user)
	return err
}

// Delete implements UserRepository
func (t *UserRepositoryImpl) Delete(id string) error {
	_, err := t.Db.Table("user").ID(id).Delete(&model.User{})
	return err
}

func (t *UserRepositoryImpl) Update(user model.User) error {
	_, err := t.Db.Table("user").ID(user.UserId).Update(&user)
	return err
}

// FindAll implements UserRepository
func (t *UserRepositoryImpl) FindAll() ([]model.User, error) {
	var users []model.User
	err := t.Db.Table("user").Find(&users)
	return users, err
}

// FindById implements UserRepository
func (t *UserRepositoryImpl) FindById(id string) (model.User, error) {
	var user model.User
	has, err := t.Db.Table("user").ID(id).Get(&user)
	if has {
		return user, nil
	} else {
		return user, err
	}
}

// FindByUsername implements UserRepository
func (t *UserRepositoryImpl) FindByUsername(username string) (model.User, error) {
	var user model.User
	re, err := t.Db.Table("user").Where("username = ?", username).Get(&user)
	if re {
		return user, nil
	} else {
		return user, err
	}
}
