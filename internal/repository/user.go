package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"gorm.io/gorm"
)

type IUserRepository interface {
	Insert(user model.User) error
	Update(user model.User) error
	Delete(id uint) error
	FindById(id uint) (user model.User, err error)
	FindByUsername(username string) (user model.User, err error)
	FindByEmail(email string) (user model.User, err error)
	Find(req request.UserFindRequest) (user []model.User, count int64, err error)
	BatchFindByTeamId(req request.UserBatchFindByTeamIdRequest) (users []model.User, err error)
}

type UserRepository struct {
	db *gorm.DB
}

func NewUserRepository(db *gorm.DB) IUserRepository {
	return &UserRepository{db: db}
}

func (t *UserRepository) Insert(user model.User) error {
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

func (t *UserRepository) Find(req request.UserFindRequest) (users []model.User, count int64, err error) {
	applyFilter := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.Email != "" {
			q = q.Where("email LIKE ?", "%"+req.Email+"%")
		}
		if req.Name != "" {
			q = q.Where("name LIKE ? OR username LIKE ?", "%"+req.Name+"%", "%"+req.Name+"%")
		}
		return q
	}
	db := applyFilter(t.db.Table("users"))
	result := db.Model(&model.User{}).Count(&count)
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Order("users." + sortKey + " ASC")
		} else if sortOrder == "desc" {
			db = db.Order("users." + sortKey + " DESC")
		}
	} else {
		db = db.Order("users.id ASC") // 默认采用 IDs 升序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}
	result = db.
		Preload("Group").
		Find(&users)
	return users, count, result.Error
}

func (t *UserRepository) BatchFindByTeamId(req request.UserBatchFindByTeamIdRequest) (users []model.User, err error) {
	err = t.db.Table("users").
		Joins("INNER JOIN user_team ON users.id = user_team.user_id").
		Where("user_team.team_id = ?", req.TeamID).
		Find(&users).Error
	return users, err
}

func (t *UserRepository) FindById(id uint) (user model.User, err error) {
	result := t.db.Table("users").
		Where("id = ?", id).
		Preload("Group").
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

func (t *UserRepository) FindByEmail(email string) (user model.User, err error) {
	result := t.db.Table("users").
		Where("email = ?", email).
		Preload("Group").
		First(&user)
	return user, result.Error
}
