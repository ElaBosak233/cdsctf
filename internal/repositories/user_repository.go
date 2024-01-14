package repositories

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"xorm.io/xorm"
)

type UserRepository interface {
	Insert(user model.User) error
	Update(user model.User) error
	Delete(id int64) error
	FindById(id int64) (user model.User, err error)
	FindByUsername(username string) (user model.User, err error)
	FindByEmail(email string) (user model.User, err error)
	Find(req request.UserFindRequest) (user []model.User, count int64, err error)
}

type UserRepositoryImpl struct {
	Db *xorm.Engine
}

func NewUserRepositoryImpl(Db *xorm.Engine) UserRepository {
	return &UserRepositoryImpl{Db: Db}
}

// Insert implements UserRepository
func (t *UserRepositoryImpl) Insert(user model.User) error {
	_, err := t.Db.Table("user").Insert(&user)
	return err
}

// Delete implements UserRepository
func (t *UserRepositoryImpl) Delete(id int64) error {
	_, err := t.Db.Table("user").ID(id).Delete(&model.User{})
	return err
}

func (t *UserRepositoryImpl) Update(user model.User) error {
	_, err := t.Db.Table("user").ID(user.UserId).Update(&user)
	return err
}

// Find implements UserRepository
func (t *UserRepositoryImpl) Find(req request.UserFindRequest) (users []model.User, count int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if req.Name != "" {
			q = q.Where("name LIKE ?", "%"+req.Name+"%")
		}
		if req.Role != 0 {
			q = q.Where("role = ?", req.Role)
		}
		return q
	}
	db := applyFilter(t.Db.Table("user"))
	ct := applyFilter(t.Db.Table("user"))
	count, err = ct.Count(&model.User{})
	if req.Page != 0 && req.Size != 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&users)
	return users, count, err
}

// FindById implements UserRepository
func (t *UserRepositoryImpl) FindById(id int64) (model.User, error) {
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

func (t *UserRepositoryImpl) FindByEmail(email string) (user model.User, err error) {
	_, err = t.Db.Table("user").Where("email = ?", email).Get(&user)
	return user, err
}
