package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type IGameRepository interface {
	Create(game model.Game) (g model.Game, err error)
	Update(game model.Game) (err error)
	Delete(req request.GameDeleteRequest) (err error)
	Find(req request.GameFindRequest) (games []model.Game, count int64, err error)
}

type GameRepository struct {
	Db *gorm.DB
}

func NewGameRepository(Db *gorm.DB) IGameRepository {
	return &GameRepository{Db: Db}
}

func (t *GameRepository) Create(game model.Game) (g model.Game, err error) {
	result := t.Db.Table("games").Create(&game)
	return game, result.Error
}

func (t *GameRepository) Update(game model.Game) (err error) {
	result := t.Db.Table("games").Model(&game).Updates(&game)
	return result.Error
}

func (t *GameRepository) Delete(req request.GameDeleteRequest) (err error) {
	result := t.Db.Table("games").Delete(&model.Game{
		ID: req.ID,
	})
	return result.Error
}

func (t *GameRepository) Find(req request.GameFindRequest) (games []model.Game, count int64, err error) {
	applyFilters := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.Title != "" {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsEnabled != nil {
			q = q.Where("is_enabled = ?", *(req.IsEnabled))
		}
		return q
	}
	db := applyFilters(t.Db.Table("games"))

	result := db.Model(&model.Game{}).Count(&count)
	if req.SortKey != "" && req.SortOrder != "" {
		db = db.Order(req.SortKey + " " + req.SortOrder)
	} else {
		db = db.Order("games.id DESC")
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	result = db.Find(&games)
	return games, count, result.Error
}
