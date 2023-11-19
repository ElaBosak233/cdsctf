package repository

import (
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/utils"
	"xorm.io/xorm"
)

type UserRepositoryImpl struct {
	Db *xorm.Engine
}

func NewUserRepositoryImpl(Db *xorm.Engine) UserRepository {
	return &UserRepositoryImpl{Db: Db}
}

// Insert implements UserRepository
func (t *UserRepositoryImpl) Insert(user model.User) error {
	_, err := t.Db.Insert(&user)
	return err
}

// Delete implements UserRepository
func (t *UserRepositoryImpl) Delete(id string) error {
	var user model.User
	_, err := t.Db.Table("user").Where("id = ?", id).Delete(&user)
	return err
}

func (t *UserRepositoryImpl) Update(user model.User) error {
	_, err := t.Db.Table("user").ID(user.Id).Update(&user)
	return err
}

// FindAll implements UserRepository
func (t *UserRepositoryImpl) FindAll() []model.User {
	var users []model.User
	err := t.Db.Find(&users)
	utils.ErrorPanic(err)
	return users
}

// FindById implements UserRepository
func (t *UserRepositoryImpl) FindById(id string) (model.User, error) {
	var user model.User
	has, err := t.Db.Table("user").Where("id = ?", id).Get(&user)
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
