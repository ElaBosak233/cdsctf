package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type EnvRepository interface {
	Insert(env entity.Env) (e entity.Env, err error)
	FindByImageID(imageIDs []int64) (envs []entity.Env, err error)
}

type EnvRepositoryImpl struct {
	Db *xorm.Engine
}

func NewEnvRepositoryImpl(Db *xorm.Engine) EnvRepository {
	return &EnvRepositoryImpl{Db: Db}
}

func (t *EnvRepositoryImpl) Insert(env entity.Env) (e entity.Env, err error) {
	_, err = t.Db.Table("env").Insert(&env)
	return env, err
}

func (t *EnvRepositoryImpl) FindByImageID(imageIDs []int64) (envs []entity.Env, err error) {
	err = t.Db.Table("env").In("image_id", imageIDs).Find(&envs)
	return envs, err
}
