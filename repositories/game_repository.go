package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"xorm.io/xorm"
)

type GameRepository interface {
	Insert(game entity.Game) (g entity.Game, err error)
	Update(game entity.Game) (err error)
	Delete(id int64) (err error)
	Find(req request.GameFindRequest) (games []response.GameResponse, count int64, err error)
}

type GameRepositoryImpl struct {
	Db *xorm.Engine
}

func NewGameRepositoryImpl(Db *xorm.Engine) GameRepository {
	return &GameRepositoryImpl{Db: Db}
}

func (t *GameRepositoryImpl) Insert(game entity.Game) (g entity.Game, err error) {
	_, err = t.Db.Table("game").Insert(&game)
	return game, err
}

func (t *GameRepositoryImpl) Update(game entity.Game) (err error) {
	_, err = t.Db.Table("game").ID(game.GameID).Update(&game)
	return err
}

func (t *GameRepositoryImpl) Delete(id int64) (err error) {
	_, err = t.Db.Table("game").Delete(&entity.Game{
		GameID: id,
	})
	return err
}

func (t *GameRepositoryImpl) Find(req request.GameFindRequest) (games []response.GameResponse, count int64, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.GameId != 0 {
			q = q.Where("id = ?", req.GameId)
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
	count, err = ct.Count(&entity.Submission{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("game." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("game." + sortKey)
		}
	} else {
		db = db.Desc("game.id") // 默认采用 ID 降序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&games)
	return games, count, err
}
