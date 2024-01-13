package repositories

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"time"
	"xorm.io/xorm"
)

type InstanceRepository interface {
	Insert(instance model.Instance) (i model.Instance, err error)
	Update(instance model.Instance) (err error)
	Find(req request.InstanceFindRequest) (instances []model.Instance, pageCount int64, err error)
	FindById(id int64) (instance model.Instance, err error)
	FindAllAvailable() (instances []model.Instance, err error)
	FindAll() (instances []model.Instance, err error)
}

type InstanceRepositoryImpl struct {
	Db *xorm.Engine
}

func NewInstanceRepositoryImpl(Db *xorm.Engine) InstanceRepository {
	return &InstanceRepositoryImpl{Db: Db}
}

func (t *InstanceRepositoryImpl) Insert(instance model.Instance) (i model.Instance, err error) {
	_, err = t.Db.Table("instance").Insert(&instance)
	return instance, err
}

func (t *InstanceRepositoryImpl) Update(instance model.Instance) (err error) {
	_, err = t.Db.Table("instance").ID(instance.InstanceId).Update(&instance)
	return err
}

func (t *InstanceRepositoryImpl) Find(req request.InstanceFindRequest) (instances []model.Instance, pageCount int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if req.ChallengeId != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeId)
		}
		if req.UserId != 0 {
			q = q.Where("user_id = ?", req.UserId)
		}
		if req.TeamId != 0 {
			q = q.Where("team_id = ?", req.TeamId)
		}
		if req.GameId != 0 {
			q = q.Where("game_id = ?", req.GameId)
		}
		if req.IsAvailable != -1 {
			if req.IsAvailable == 0 {
				q = q.Where("removed_at < ?", time.Now().Unix())
			} else if req.IsAvailable == 1 {
				q = q.Where("removed_at > ?", time.Now().Unix())
			}
		}
		return q
	}
	db := applyFilter(t.Db.Table("instance"))
	count, err := applyFilter(t.Db.Table("instance")).Count(&model.Instance{})
	if req.Page != -1 && req.Size != -1 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&instances)
	return instances, count, err
}

func (t *InstanceRepositoryImpl) FindById(id int64) (instance model.Instance, err error) {
	_, err = t.Db.Table("instance").ID(id).Get(&instance)
	return instance, err
}

func (t *InstanceRepositoryImpl) FindAllAvailable() (instances []model.Instance, err error) {
	err = t.Db.Table("instance").Where("removed_at > ?", time.Now().Unix()).Find(&instances)
	return instances, err
}

func (t *InstanceRepositoryImpl) FindAll() (instances []model.Instance, err error) {
	err = t.Db.Table("instance").Find(&instances)
	return instances, err
}
