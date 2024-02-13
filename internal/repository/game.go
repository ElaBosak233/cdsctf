package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	"xorm.io/xorm"
)

type IGameRepository interface {
	Insert(game model.Game) (g model.Game, err error)
	Update(game model.Game) (err error)
	Delete(id int64) (err error)
	Find(req request.GameFindRequest) (games []response.GameResponse, count int64, err error)
}

type GameRepository struct {
	Db *xorm.Engine
}

func NewGameRepository(Db *xorm.Engine) IGameRepository {
	return &GameRepository{Db: Db}
}

func (t *GameRepository) Insert(game model.Game) (g model.Game, err error) {
	_, err = t.Db.Table("game").Insert(&game)
	return game, err
}

func (t *GameRepository) Update(game model.Game) (err error) {
	_, err = t.Db.Table("game").ID(game.ID).Update(&game)
	return err
}

func (t *GameRepository) Delete(id int64) (err error) {
	_, err = t.Db.Table("game").Delete(&model.Game{
		ID: id,
	})
	return err
}

func (t *GameRepository) Find(req request.GameFindRequest) (games []response.GameResponse, count int64, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.Title != "" {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsEnabled != -1 {
			q = q.Where("is_enabled = ?", req.IsEnabled == 1)
		}
		return q
	}
	db := applyFilters(t.Db.Table("game"))
	ct := applyFilters(t.Db.Table("game"))
	count, err = ct.Count(&model.Submission{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("game." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("game." + sortKey)
		}
	} else {
		db = db.Desc("game.id") // 默认采用 IDs 降序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&games)
	return games, count, err
}
