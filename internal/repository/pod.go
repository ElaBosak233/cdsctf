package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"gorm.io/gorm"
	"time"
)

type IPodRepository interface {
	Insert(pod model.Pod) (i model.Pod, err error)
	Update(pod model.Pod) (err error)
	Find(req request.PodFindRequest) (pods []model.Pod, pageCount int64, err error)
	FindById(id uint) (pod model.Pod, err error)
}

type PodRepository struct {
	Db *gorm.DB
}

func NewPodRepository(Db *gorm.DB) IPodRepository {
	return &PodRepository{Db: Db}
}

func (t *PodRepository) Insert(pod model.Pod) (i model.Pod, err error) {
	result := t.Db.Table("pods").Create(&pod)
	return pod, result.Error
}

func (t *PodRepository) Update(pod model.Pod) (err error) {
	result := t.Db.Table("pods").Model(&pod).Updates(&pod)
	return result.Error
}

func (t *PodRepository) Find(req request.PodFindRequest) (pods []model.Pod, pageCount int64, err error) {
	applyFilter := func(q *gorm.DB) *gorm.DB {
		if req.ChallengeID != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeID)
		}
		if req.UserID != 0 {
			q = q.Where("user_id = ?", req.UserID)
		}
		if req.TeamID != nil {
			q = q.Where("team_id = ?", *(req.TeamID))
		}
		if req.GameID != nil {
			q = q.Where("game_id = ?", *(req.GameID))
		}
		if req.IsAvailable != nil {
			if *(req.IsAvailable) == false {
				q = q.Where("removed_at < ?", time.Now().Unix())
			} else if *(req.IsAvailable) == true {
				q = q.Where("removed_at > ?", time.Now().Unix())
			}
		}
		if len(req.IDs) > 0 {
			q = q.Where("id IN ?", req.IDs)
		}
		return q
	}
	db := applyFilter(t.Db.Table("pods"))
	result := applyFilter(t.Db.Table("pods")).Model(&model.Pod{}).Count(&pageCount)
	if req.Page != 0 && req.Size != 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}
	result = db.
		Preload("Instances.Image").
		Preload("Instances.Nats").
		Preload("Instances.Image.Ports").
		Preload("Instances.Image.Envs").
		Find(&pods)
	return pods, pageCount, result.Error
}

func (t *PodRepository) FindById(id uint) (pod model.Pod, err error) {
	result := t.Db.Table("pods").First(&model.Pod{ID: id})
	return pod, result.Error
}
