package initialize

import (
	"fmt"
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/entity/relations"
	"github.com/elabosak233/pgshub/utils"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
	"github.com/spf13/viper"
	"github.com/xormplus/xorm"
	"golang.org/x/crypto/bcrypt"
	"time"
)

var db *xorm.Engine
var dbInfo string

// GetDatabaseConnection 获取数据库连接
func GetDatabaseConnection() *xorm.Engine {
	InitDatabaseEngine()
	utils.Logger.Info("数据库连接信息 " + dbInfo)
	SyncDatabase()
	InitAdmin()
	SelfCheck()
	return db
}

// InitDatabaseEngine 初始化数据库引擎
func InitDatabaseEngine() {
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
		db, err = xorm.NewPostgreSQL(dbInfo)
	} else if viper.GetString("db.provider") == "sqlite3" {
		dbInfo = viper.GetString("db.sqlite3.filename")
		db, err = xorm.NewSqlite3(dbInfo)
	}
	if err != nil {
		utils.Logger.Error("数据库连接失败")
		panic(err)
	}
}

// SyncDatabase 同步数据库
func SyncDatabase() {
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
func SelfCheck() {
	// 对于 instances 中的所有数据，若 removed_at 大于当前时间，则强制赋值为现在的时间，以免后续程序错误判断
	_, _ = db.Table("instances").Where("removed_at > ?", time.Now()).Update(entity.Instance{
		RemovedAt: time.Now(),
	})
}

// InitAdmin 创建超级管理员账户
// 仅用于第一次生成
func InitAdmin() {
	existAdminUser, _ := db.Table("users").Where("username = ?", "admin").Exist()
	if !existAdminUser {
		utils.Logger.Warn("超级管理员账户不存在，即将创建")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("123456"), bcrypt.DefaultCost)
		_, err := db.Table("users").Insert(entity.User{
			Username: "admin",
			Nickname: "超级管理员",
			Role:     1,
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		})
		if err != nil {
			utils.Logger.Error("超级管理员账户创建失败")
			panic(err)
			return
		}
		utils.Logger.Infof("超级管理员账户创建成功")
	}
}
