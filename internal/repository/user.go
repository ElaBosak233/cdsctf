package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	"xorm.io/xorm"
)

type IUserRepository interface {
	Insert(user model.User) error
	Update(user model.User) error
	Delete(id int64) error
	FindById(id int64) (user model.User, err error)
	FindByUsername(username string) (user model.User, err error)
	FindByEmail(email string) (user model.User, err error)
	Find(req request.UserFindRequest) (user []response.UserResponse, count int64, err error)
	BatchFindByTeamId(req request.UserBatchFindByTeamIdRequest) (users []response.UserResponseWithTeamId, err error)
}

type UserRepository struct {
	Db *xorm.Engine
}

func NewUserRepository(Db *xorm.Engine) IUserRepository {
	return &UserRepository{Db: Db}
}

func (t *UserRepository) Insert(user model.User) error {
	_, err := t.Db.Table("account").Insert(&user)
	return err
}

func (t *UserRepository) Delete(id int64) error {
	_, err := t.Db.Table("account").ID(id).Delete(&model.User{})
	return err
}

func (t *UserRepository) Update(user model.User) error {
	_, err := t.Db.Table("account").ID(user.ID).Update(&user)
	return err
}

func (t *UserRepository) Find(req request.UserFindRequest) (users []response.UserResponse, count int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
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
	db := applyFilter(t.Db.Table("account"))
	ct := applyFilter(t.Db.Table("account"))
	count, err = ct.Count(&model.User{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("account." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("account." + sortKey)
		}
	} else {
		db = db.Asc("account.id") // 默认采用 IDs 升序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&users)
	return users, count, err
}

func (t *UserRepository) BatchFindByTeamId(req request.UserBatchFindByTeamIdRequest) (users []response.UserResponseWithTeamId, err error) {
	err = t.Db.Table("account").
		Join("INNER", "user_team", "account.id = user_team.user_id").
		In("user_team.team_id", req.TeamID).
		Find(&users)
	return users, err
}

func (t *UserRepository) FindById(id int64) (user model.User, err error) {
	_, err = t.Db.Table("account").ID(id).Get(&user)
	return user, err
}

func (t *UserRepository) FindByUsername(username string) (user model.User, err error) {
	_, err = t.Db.Table("account").Where("username = ?", username).Get(&user)
	return user, err
}

func (t *UserRepository) FindByEmail(email string) (user model.User, err error) {
	_, err = t.Db.Table("account").Where("email = ?", email).Get(&user)
	return user, err
}
