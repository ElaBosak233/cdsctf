package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
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
	Db *gorm.DB
}

func NewUserRepository(Db *gorm.DB) IUserRepository {
	return &UserRepository{Db: Db}
}

func (t *UserRepository) Insert(user model.User) error {
	result := t.Db.Table("users").Create(&user)
	return result.Error
}

func (t *UserRepository) Delete(id uint) error {
	result := t.Db.Table("users").Delete(&model.User{
		ID: id,
	})
	return result.Error
}

func (t *UserRepository) Update(user model.User) error {
	result := t.Db.Table("users").Model(&user).Updates(&user)
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
		if req.Role != 0 {
			q = q.Where("role = ?", req.Role)
		}
		return q
	}
	db := applyFilter(t.Db.Table("users"))
	ct := applyFilter(t.Db.Table("users"))
	result := ct.Model(&model.User{}).Count(&count)
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
	result = db.Find(&users)
	return users, count, result.Error
}

func (t *UserRepository) BatchFindByTeamId(req request.UserBatchFindByTeamIdRequest) (users []model.User, err error) {
	err = t.Db.Table("users").
		Joins("INNER JOIN user_team ON users.id = user_team.user_id").
		Where("user_team.team_id = ?", req.TeamID).
		Find(&users).Error
	return users, err
}

func (t *UserRepository) FindById(id uint) (user model.User, err error) {
	result := t.Db.Table("users").Where("id = ?", id).First(&user)
	return user, result.Error
}

func (t *UserRepository) FindByUsername(username string) (user model.User, err error) {
	result := t.Db.Table("users").Where("username = ?", username).First(&user)
	return user, result.Error
}

func (t *UserRepository) FindByEmail(email string) (user model.User, err error) {
	result := t.Db.Table("users").Where("email = ?", email).First(&user)
	return user, result.Error
}
