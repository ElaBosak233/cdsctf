package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IEnvRepository interface {
	Insert(env model.Env) (e model.Env, err error)
	FindByImageID(imageIDs []uint) (envs []model.Env, err error)
}

type EnvRepository struct {
	db *gorm.DB
}

func NewEnvRepository(db *gorm.DB) IEnvRepository {
	return &EnvRepository{db: db}
}

func (t *EnvRepository) Insert(env model.Env) (e model.Env, err error) {
	result := t.db.Table("envs").
		Create(&env)
	return env, result.Error
}

func (t *EnvRepository) FindByImageID(imageIDs []uint) (envs []model.Env, err error) {
	result := t.db.Table("envs").
		Where("image_id IN ?", imageIDs).
		Find(&envs)
	return envs, result.Error
}
