package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
	"time"
)

type IPodRepository interface {
	Create(pod model.Pod) (model.Pod, error)
	Update(pod model.Pod) error
	Find(req request.PodFindRequest) ([]model.Pod, int64, error)
}

type PodRepository struct {
	db *gorm.DB
}

func NewPodRepository(db *gorm.DB) IPodRepository {
	return &PodRepository{db: db}
}

func (t *PodRepository) Create(pod model.Pod) (model.Pod, error) {
	result := t.db.Table("pods").Create(&pod)
	return pod, result.Error
}

func (t *PodRepository) Update(pod model.Pod) error {
	result := t.db.Table("pods").Model(&pod).Updates(&pod)
	return result.Error
}

func (t *PodRepository) Find(req request.PodFindRequest) ([]model.Pod, int64, error) {
	var pods []model.Pod
	applyFilter := func(q *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			q = q.Where("id = ?", req.ID)
		}
		if req.ChallengeID != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeID)
		}
		if req.UserID != nil {
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
		return q
	}
	db := applyFilter(t.db.Table("pods"))
	var total int64 = 0
	result := db.Model(&model.Pod{}).Count(&total)
	if req.Page != 0 && req.Size != 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	result = db.
		Preload("Challenge", func(db *gorm.DB) *gorm.DB {
			return db.
				Preload("Ports").
				Preload("Envs").
				Select([]string{"id", "title"})
		}).
		Preload("Nats").
		Find(&pods)
	return pods, total, result.Error
}
