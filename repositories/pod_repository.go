package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"time"
	"xorm.io/xorm"
)

type PodRepository interface {
	Insert(pod entity.Pod) (i entity.Pod, err error)
	Update(pod entity.Pod) (err error)
	Find(req request.PodFindRequest) (pods []entity.Pod, pageCount int64, err error)
	FindById(id int64) (pod entity.Pod, err error)
}

type PodRepositoryImpl struct {
	Db *xorm.Engine
}

func NewPodRepositoryImpl(Db *xorm.Engine) PodRepository {
	return &PodRepositoryImpl{Db: Db}
}

func (t *PodRepositoryImpl) Insert(pod entity.Pod) (i entity.Pod, err error) {
	_, err = t.Db.Table("pod").Insert(&pod)
	return pod, err
}

func (t *PodRepositoryImpl) Update(pod entity.Pod) (err error) {
	_, err = t.Db.Table("pod").ID(pod.ID).Update(&pod)
	return err
}

func (t *PodRepositoryImpl) Find(req request.PodFindRequest) (pods []entity.Pod, pageCount int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if req.ChallengeID != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeID)
		}
		if req.UserID != 0 {
			q = q.Where("user_id = ?", req.UserID)
		}
		if req.TeamID != 0 {
			q = q.Where("team_id = ?", req.TeamID)
		}
		if req.GameID != 0 {
			q = q.Where("game_id = ?", req.GameID)
		}
		if req.IsAvailable != nil {
			if *(req.IsAvailable) == false {
				q = q.Where("removed_at < ?", time.Now().Unix())
			} else if *(req.IsAvailable) == true {
				q = q.Where("removed_at > ?", time.Now().Unix())
			}
		}
		if len(req.IDs) > 0 {
			q = q.In("id", req.IDs)
		}
		return q
	}
	db := applyFilter(t.Db.Table("pod"))
	count, err := applyFilter(t.Db.Table("pod")).Count(&entity.Pod{})
	if req.Page != 0 && req.Size != 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&pods)
	return pods, count, err
}

func (t *PodRepositoryImpl) FindById(id int64) (pod entity.Pod, err error) {
	_, err = t.Db.Table("pod").ID(id).Get(&pod)
	return pod, err
}
