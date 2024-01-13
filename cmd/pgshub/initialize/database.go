package initialize

import (
	"fmt"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/data/relations"
	"github.com/elabosak233/pgshub/internal/utils"
	_ "github.com/go-sql-driver/mysql"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
	"github.com/spf13/viper"
	"golang.org/x/crypto/bcrypt"
	"xorm.io/xorm"
)

var db *xorm.Engine
var dbInfo string

func GetDatabaseConnection() *xorm.Engine {
	InitDatabaseEngine()
	utils.Logger.Info("数据库连接信息 " + dbInfo)
	SyncDatabase()
	InitAdmin()
	return db
}

func InitDatabaseEngine() {
	var err error
	if viper.GetString("db.provider") == "mysql" {
		dbInfo = fmt.Sprintf(
			"%s:%s@tcp(%s:%d)/%s?charset=utf8mb4",
			viper.GetString("db.mysql.username"),
			viper.GetString("db.mysql.password"),
			viper.GetString("db.mysql.host"),
			viper.GetInt("db.mysql.port"),
			viper.GetString("db.mysql.dbname"),
		)
		db, err = xorm.NewEngine("mysql", dbInfo)
	} else if viper.GetString("db.provider") == "postgres" {
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
	} else if viper.GetString("db.provider") == "sqlite" {
		dbInfo = viper.GetString("db.sqlite.filename")
		db, err = xorm.NewEngine("sqlite3", dbInfo)
	}
	if err != nil {
		utils.Logger.Error("数据库连接失败")
		panic(err)
	}
}

func SyncDatabase() {
	var dbs = []interface{}{
		&model.User{},
		&model.Challenge{},
		&model.Team{},
		&relations.UserTeam{},
		&model.Submission{},
		&model.Instance{},
	}
	for _, v := range dbs {
		err := db.Sync2(v)
		if err != nil {
			panic(err)
		}
	}
}

func InitAdmin() {
	existAdminUser, _ := db.Table("user").Where("username = ?", "admin").Exist()
	if !existAdminUser {
		utils.Logger.Warn("超级管理员账户不存在，即将创建")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("admin"), bcrypt.DefaultCost)
		_, err := db.Table("user").Insert(model.User{
			Username: "admin",
			Name:     "超级管理员",
			Role:     0,
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
