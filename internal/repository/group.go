package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type IGroupRepository interface {
	Find(req request.GroupFindRequest) (groups []model.Group, err error)
	Update(req request.GroupUpdateRequest) (err error)
}

type GroupRepository struct {
	db *gorm.DB
}

func NewGroupRepository(db *gorm.DB) IGroupRepository {
	return &GroupRepository{db: db}
}

func (t *GroupRepository) Find(req request.GroupFindRequest) (groups []model.Group, err error) {
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.Name != "" {
			q = q.Where("display_name LIKE ? or name LIKE ?", "%"+req.Name+"%", "%"+req.Name+"%")
		}
		return q
	}
	db := applyFilters(t.db.Table("groups"))

	result := db.Find(&groups)
	return groups, result.Error
}

func (t *GroupRepository) Update(req request.GroupUpdateRequest) (err error) {
	result := t.db.Table("groups").Where("id = ?", req.ID).Updates(&model.Group{
		DisplayName: req.DisplayName,
		Description: req.Description,
	})
	return result.Error
}
