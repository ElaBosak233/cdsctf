package database

import (
	"fmt"
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/entity/relations"
	"github.com/elabosak233/pgshub/utils/logger"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
	"github.com/spf13/viper"
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
	selfCheck()
}

func GetDatabase() *xorm.Engine {
	return db
}

// initDatabaseEngine 初始化数据库引擎
func initDatabaseEngine() {
	var err error
	if viper.GetString("db.provider") == "postgres" {
		dbInfo = fmt.Sprintf(
			"host=%s port=%d user=%s password=%s dbname=%s sslmode=%s",
			viper.GetString("db.postgres.host"),
			viper.GetInt("db.postgres.port"),
			viper.GetString("db.postgres.username"),
			viper.GetString("db.postgres.password"),
			viper.GetString("db.postgres.dbname"),
			viper.GetString("db.postgres.sslmode"),
		)
		db, err = xorm.NewEngine("postgres", dbInfo)
	} else if viper.GetString("db.provider") == "sqlite3" {
		dbInfo = viper.GetString("db.sqlite3.filename")
		db, err = xorm.NewEngine("sqlite3", dbInfo)
	}
	if err != nil {
		zap.L().Error("Database connection failed.")
		panic(err)
	}
}

// SyncDatabase 同步数据库
func syncDatabase() {
	var dbs = []interface{}{
		&entity.User{},
		&entity.Challenge{},
		&entity.Team{},
		&relations.UserTeam{},
		&entity.Submission{},
		&entity.Instance{},
		&entity.Game{},
		&relations.GameChallenge{},
	}
	for _, v := range dbs {
		err := db.Sync2(v)
		if err != nil {
			panic(err)
		}
	}
}

// SelfCheck 数据库自检
// 主要用于配平不合理的时间数据
func selfCheck() {
	// 对于 instances 中的所有数据，若 removed_at 大于当前时间，则强制赋值为现在的时间，以免后续程序错误判断
	_, _ = db.Table("instance").Where("removed_at > ?", time.Now().UTC()).Update(entity.Instance{
		RemovedAt: time.Now().UTC(),
	})
}

// InitAdmin 创建超级管理员账户
// 仅用于第一次生成
func initAdmin() {
	existAdminUser, _ := db.Table("user").Where("username = ?", "admin").Exist()
	if !existAdminUser {
		zap.L().Warn("Administrator account does not exist, will be created soon.")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("123456"), bcrypt.DefaultCost)
		_, err := db.Table("user").Insert(entity.User{
			Username: "admin",
			Nickname: "Administrator",
			Role:     1,
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		})
		if err != nil {
			zap.L().Error("Super administrator account creation failed.")
			panic(err)
			return
		}
		zap.L().Info("Super administrator account created successfully.")
	}
}
