package casbin

import (
	"github.com/casbin/casbin/v2"
	"github.com/casbin/casbin/v2/model"
	gormadapter "github.com/casbin/gorm-adapter/v3"
	"github.com/elabosak233/cloudsdale/embed"
	"github.com/elabosak233/cloudsdale/internal/database"
	"go.uber.org/zap"
)

var (
	Enforcer *casbin.Enforcer
)

func InitCasbin() {
	adapter, err := gormadapter.NewAdapterByDB(database.Db())
	cfg, err := embed.FS.ReadFile("configs/casbin.conf")
	md, _ := model.NewModelFromString(string(cfg))
	Enforcer, err = casbin.NewEnforcer(md, adapter)
	if err != nil {
		zap.L().Fatal("Casbin init failed", zap.Error(err))
	}
	_ = Enforcer.LoadPolicy()
	initDefaultPolicy()
}
