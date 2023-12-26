package implements

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
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

// Find implements UserRepository
func (t *UserRepositoryImpl) Find(req request.UserFindRequest) (users []model.User, count int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if req.Name != "" {
			q = q.Where("name LIKE ?", "%"+req.Name+"%")
		}
		if req.Role != -1 {
			q = q.Where("role = ?", req.Role)
		}
		return q
	}
	db := applyFilter(t.Db.Table("user"))
	ct := applyFilter(t.Db.Table("user"))
	count, err = ct.Count(&model.User{})
	if req.Page != -1 && req.Size != -1 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&users)
	return users, count, err
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

func (t *UserRepositoryImpl) FindByEmail(email string) (user model.User, err error) {
	_, err = t.Db.Table("user").Where("email = ?", email).Get(&user)
	return user, err
}
