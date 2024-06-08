package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IEnvRepository interface {
	Create(env model.Env) (model.Env, error)
}

type EnvRepository struct {
	db *gorm.DB
}

func NewEnvRepository(db *gorm.DB) IEnvRepository {
	return &EnvRepository{db: db}
}

func (t *EnvRepository) Create(env model.Env) (model.Env, error) {
	result := t.db.Table("envs").
		Create(&env)
	return env, result.Error
}
