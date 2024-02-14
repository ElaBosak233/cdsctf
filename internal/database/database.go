package database

import (
	"fmt"
	"github.com/elabosak233/pgshub/internal/config"
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/pkg/logger"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
	"go.uber.org/zap"
	"golang.org/x/crypto/bcrypt"
	"time"
	"xorm.io/xorm"
)

var db *xorm.Engine
var dbInfo string

func InitDatabase() {
	initDatabaseEngine()
	zap.L().Info(fmt.Sprintf("Database Connect Information: %s", dbInfo))
	db.SetLogger(logger.Logger(zap.L()))
	syncDatabase()
	initAdmin()
	initCategory()
	selfCheck()
}

func GetDatabase() *xorm.Engine {
	return db
}

// initDatabaseEngine 初始化数据库引擎
func initDatabaseEngine() {
	var err error
	if config.AppCfg().Db.Provider == "postgres" {
		dbInfo = fmt.Sprintf(
			"host=%s port=%d user=%s password=%s dbname=%s sslmode=%s",
			config.AppCfg().Db.Postgres.Host,
			config.AppCfg().Db.Postgres.Port,
			config.AppCfg().Db.Postgres.Username,
			config.AppCfg().Db.Postgres.Password,
			config.AppCfg().Db.Postgres.Dbname,
			config.AppCfg().Db.Postgres.Sslmode,
		)
		db, err = xorm.NewEngine("postgres", dbInfo)
	} else if config.AppCfg().Db.Provider == "sqlite3" {
		dbInfo = config.AppCfg().Db.Sqlite3.Filename
		db, err = xorm.NewEngine("sqlite3", dbInfo)
	}
	if err != nil {
		zap.L().Fatal("Database connection failed.", zap.Error(err))
	}
}

// SyncDatabase 同步数据库
func syncDatabase() {
	var dbs = []interface{}{
		&model.User{},
		&model.Category{},
		&model.Challenge{},
		&model.Team{},
		&model.UserTeam{},
		&model.Submission{},
		&model.Nat{},
		&model.Instance{},
		&model.Pod{},
		&model.Game{},
		&model.GameChallenge{},
		&model.Image{},
		&model.Flag{},
		&model.FlagGen{},
		&model.Port{},
		&model.Nat{},
		&model.Env{},
	}
	for _, v := range dbs {
		err := db.Sync2(v)
		if err != nil {
			zap.L().Fatal("Database sync failed.", zap.Error(err))
		}
	}
}

// SelfCheck 数据库自检
// 主要用于配平不合理的时间数据
func selfCheck() {
	// 对于 pods 中的所有数据，若 removed_at 大于当前时间，则强制赋值为现在的时间，以免后续程序错误判断
	_, _ = db.Table("pod").Where("removed_at > ?", time.Now().Unix()).Update(model.Pod{
		RemovedAt: time.Now().Unix(),
	})
}

// InitAdmin 创建超级管理员账户
// 仅用于第一次生成
func initAdmin() {
	existAdminUser, _ := db.Table("account").Where("username = ?", "admin").Exist()
	if !existAdminUser {
		zap.L().Warn("Administrator account does not exist, will be created soon.")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("123456"), bcrypt.DefaultCost)
		_, err := db.Table("account").Insert(model.User{
			Username: "admin",
			Nickname: "Administrator",
			Role:     1,
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		})
		if err != nil {
			zap.L().Fatal("Super administrator account creation failed.", zap.Error(err))
			return
		}
		zap.L().Info("Super administrator account created successfully.")
	}
}

func initCategory() {
	count, _ := db.Table("category").Count(&model.Category{})
	if count == 0 {
		defaultCategories := []model.Category{
			{
				Name:        "misc",
				Description: "misc",
				Color:       "#3F51B5",
				Icon:        "fingerprint",
			},
			{
				Name:        "web",
				Description: "web",
				Color:       "#009688",
				Icon:        "web",
			},
			{
				Name:        "pwn",
				Description: "pwn",
				Color:       "#673AB7",
				Icon:        "matrix",
			},
			{
				Name:        "crypto",
				Description: "crypto",
				Color:       "#607D8B",
				Icon:        "pound",
			},
			{
				Name:        "reverse",
				Description: "reverse",
				Color:       "#009688",
				Icon:        "chevron-triple-left",
			},
		}
		_, err := db.Table("category").Insert(&defaultCategories)
		if err != nil {
			zap.L().Fatal("Category initialization failed.", zap.Error(err))
			return
		}
	}
}
