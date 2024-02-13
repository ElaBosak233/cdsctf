package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"time"
	"xorm.io/xorm"
)

type IPodRepository interface {
	Insert(pod model.Pod) (i model.Pod, err error)
	Update(pod model.Pod) (err error)
	Find(req request.PodFindRequest) (pods []model.Pod, pageCount int64, err error)
	FindById(id int64) (pod model.Pod, err error)
}

type PodRepository struct {
	Db *xorm.Engine
}

func NewPodRepository(Db *xorm.Engine) IPodRepository {
	return &PodRepository{Db: Db}
}

func (t *PodRepository) Insert(pod model.Pod) (i model.Pod, err error) {
	_, err = t.Db.Table("pod").Insert(&pod)
	return pod, err
}

func (t *PodRepository) Update(pod model.Pod) (err error) {
	_, err = t.Db.Table("pod").ID(pod.ID).Update(&pod)
	return err
}

func (t *PodRepository) Find(req request.PodFindRequest) (pods []model.Pod, pageCount int64, err error) {
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
	count, err := applyFilter(t.Db.Table("pod")).Count(&model.Pod{})
	if req.Page != 0 && req.Size != 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&pods)
	return pods, count, err
}

func (t *PodRepository) FindById(id int64) (pod model.Pod, err error) {
	_, err = t.Db.Table("pod").ID(id).Get(&pod)
	return pod, err
}
