package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IEnvRepository interface {
	Insert(env model.Env) (e model.Env, err error)
	FindByImageID(imageIDs []int64) (envs []model.Env, err error)
}

type EnvRepository struct {
	Db *xorm.Engine
}

func NewEnvRepository(Db *xorm.Engine) IEnvRepository {
	return &EnvRepository{Db: Db}
}

func (t *EnvRepository) Insert(env model.Env) (e model.Env, err error) {
	_, err = t.Db.Table("env").Insert(&env)
	return env, err
}

func (t *EnvRepository) FindByImageID(imageIDs []int64) (envs []model.Env, err error) {
	err = t.Db.Table("env").In("image_id", imageIDs).Find(&envs)
	return envs, err
}
