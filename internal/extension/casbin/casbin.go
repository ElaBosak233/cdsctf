package casbin

import (
	"github.com/casbin/casbin/v2"
	"github.com/casbin/casbin/v2/model"
	gormadapter "github.com/casbin/gorm-adapter/v3"
	"github.com/elabosak233/cloudsdale/internal/extension/database"
	"github.com/elabosak233/cloudsdale/internal/extension/files"
	"go.uber.org/zap"
)

var (
	Enforcer *casbin.Enforcer
)

func InitCasbin() {
	adapter, err := gormadapter.NewAdapterByDBWithCustomTable(
		database.Db(),
		&gormadapter.CasbinRule{},
		"casbins",
	)
	cfg, err := files.FS.ReadFile("configs/casbin.conf")
	md, _ := model.NewModelFromString(string(cfg))
	Enforcer, err = casbin.NewEnforcer(md, adapter)
	if err != nil {
		zap.L().Fatal("Casbin module inits failed.", zap.Error(err))
	}
	Enforcer.ClearPolicy()
	_ = Enforcer.SavePolicy()
	initDefaultPolicy()
	zap.L().Info("Casbin module inits successfully.")
}
