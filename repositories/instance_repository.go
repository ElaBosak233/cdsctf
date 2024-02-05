package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"time"
	"xorm.io/xorm"
)

type InstanceRepository interface {
	Insert(instance entity.Instance) (i entity.Instance, err error)
	Update(instance entity.Instance) (err error)
	BatchDeactivate(id []int64) (err error)
	Find(req request.InstanceFindRequest) (instances []entity.Instance, pageCount int64, err error)
	FindById(id int64) (instance entity.Instance, err error)
}

type InstanceRepositoryImpl struct {
	Db *xorm.Engine
}

func NewInstanceRepositoryImpl(Db *xorm.Engine) InstanceRepository {
	return &InstanceRepositoryImpl{Db: Db}
}

func (t *InstanceRepositoryImpl) Insert(instance entity.Instance) (i entity.Instance, err error) {
	_, err = t.Db.Table("instance").Insert(&instance)
	return instance, err
}

func (t *InstanceRepositoryImpl) Update(instance entity.Instance) (err error) {
	_, err = t.Db.Table("instance").ID(instance.InstanceID).Update(&instance)
	return err
}

func (t *InstanceRepositoryImpl) BatchDeactivate(id []int64) (err error) {
	_, err = t.Db.Table("instance").In("instance.id", id).Update(entity.Instance{
		RemovedAt: time.Now(),
	})
	return err
}

func (t *InstanceRepositoryImpl) Find(req request.InstanceFindRequest) (instances []entity.Instance, pageCount int64, err error) {
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
		if req.IsAvailable != 0 {
			if req.IsAvailable == 2 { // 无效
				q = q.Where("removed_at < ?", time.Now().UTC())
			} else if req.IsAvailable == 1 { // 有效
				q = q.Where("removed_at > ?", time.Now().UTC())
			}
		}
		return q
	}
	db := applyFilter(t.Db.Table("instance"))
	count, err := applyFilter(t.Db.Table("instance")).Count(&entity.Instance{})
	if req.Page != 0 && req.Size != 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&instances)
	return instances, count, err
}

func (t *InstanceRepositoryImpl) FindById(id int64) (instance entity.Instance, err error) {
	_, err = t.Db.Table("instance").ID(id).Get(&instance)
	return instance, err
}
