package repository

import (
	"github.com/elabosak233/pgshub/internal/model/data"
	"github.com/elabosak233/pgshub/internal/utils"
	"xorm.io/xorm"
)

type UserRepositoryImpl struct {
	Db *xorm.Engine
}

func NewUserRepositoryImpl(Db *xorm.Engine) UserRepository {
	return &UserRepositoryImpl{Db: Db}
}

// Delete implements UserRepository
func (t *UserRepositoryImpl) Delete(id string) {
	var user data.User
	_, err := t.Db.Table("user").Where("id = ?", id).Delete(&user)
	utils.ErrorPanic(err)
}

// FindAll implements UserRepository
func (t *UserRepositoryImpl) FindAll() []data.User {
	var users []data.User
	err := t.Db.Find(&users)
	utils.ErrorPanic(err)
	return users
}

// FindById implements UserRepository
func (t *UserRepositoryImpl) FindById(id string) (data.User, error) {
	var user data.User
	re, err := t.Db.Table("user").Where("id = ?", id).Get(&user)
	if re {
		return user, nil
	} else {
		return user, err
	}
}

// FindByUsername implements UserRepository
func (t *UserRepositoryImpl) FindByUsername(username string) (data.User, error) {
	var user data.User
	re, err := t.Db.Table("user").Where("username = ?", username).Get(&user)
	if re {
		return user, nil
	} else {
		return user, err
	}
}

// Save implements UserRepository
func (t *UserRepositoryImpl) Save(user data.User) {
	_, err := t.Db.Insert(&user)
	utils.ErrorPanic(err)
}

func (t *UserRepositoryImpl) Update(user data.User) {
	_, err := t.Db.Table("user").ID(user.Id).Update(&user)
	utils.ErrorPanic(err)
}
