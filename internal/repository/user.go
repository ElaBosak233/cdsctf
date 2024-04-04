package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type IUserRepository interface {
	Create(user model.User) error
	Update(user model.User) error
	Delete(id uint) error
	FindById(id uint) (user model.User, err error)
	FindByUsername(username string) (user model.User, err error)
	Find(req request.UserFindRequest) (user []model.User, total int64, err error)
}

type UserRepository struct {
	db *gorm.DB
}

func NewUserRepository(db *gorm.DB) IUserRepository {
	return &UserRepository{db: db}
}

func (t *UserRepository) Create(user model.User) error {
	result := t.db.Table("users").Create(&user)
	return result.Error
}

func (t *UserRepository) Delete(id uint) error {
	result := t.db.Table("users").Delete(&model.User{
		ID: id,
	})
	return result.Error
}

func (t *UserRepository) Update(user model.User) error {
	result := t.db.Table("users").Model(&user).Updates(&user)
	return result.Error
}

func (t *UserRepository) Find(req request.UserFindRequest) (users []model.User, total int64, err error) {
	applyFilter := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.Email != "" {
			q = q.Where("email LIKE ?", "%"+req.Email+"%")
		}
		if req.Name != "" {
			q = q.Where("nickname LIKE ? OR username LIKE ?", "%"+req.Name+"%", "%"+req.Name+"%")
		}
		return q
	}
	db := applyFilter(t.db.Table("users"))
	result := db.Model(&model.User{}).Count(&total)
	if req.SortKey != "" && req.SortOrder != "" {
		db = db.Order(req.SortKey + " " + req.SortOrder)
	} else {
		db = db.Order("users.id ASC")
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}
	result = db.
		Preload("Group").
		Preload("Teams").
		Find(&users)
	return users, total, result.Error
}

func (t *UserRepository) FindById(id uint) (user model.User, err error) {
	result := t.db.Table("users").
		Where("id = ?", id).
		Preload("Group").
		Preload("Teams").
		First(&user)
	return user, result.Error
}

func (t *UserRepository) FindByUsername(username string) (user model.User, err error) {
	result := t.db.Table("users").
		Where("username = ?", username).
		Preload("Group").
		Preload("Teams").
		First(&user)
	return user, result.Error
}
