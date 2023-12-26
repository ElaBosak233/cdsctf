package main

import (
	"fmt"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/data/relations"
	"github.com/elabosak233/pgshub/internal/utils"
	_ "github.com/go-sql-driver/mysql"
	"github.com/google/uuid"
	"github.com/spf13/viper"
	"golang.org/x/crypto/bcrypt"
	"os"
	"xorm.io/xorm"
)

func GetDatabaseConnection() *xorm.Engine {
	host := viper.GetString("db.mysql.host")
	port := viper.GetInt("db.mysql.port")
	user := viper.GetString("db.mysql.username")
	password := viper.GetString("db.mysql.password")
	dbName := viper.GetString("db.mysql.dbname")

	dbInfo := fmt.Sprintf(
		"%s:%s@tcp(%s:%d)/%s?charset=utf8mb4",
		user,
		password,
		host,
		port,
		dbName,
	)

	utils.Logger.Info("数据库连接信息 " + dbInfo)

	db, _ := xorm.NewEngine("mysql", dbInfo)
	err := SyncDatabase(db)
	if err != nil {
		utils.Logger.Error("数据库连接失败")
		os.Exit(1)
		return nil
	}
	InitAdmin(db)
	return db
}

func SyncDatabase(db *xorm.Engine) error {
	var dbs = []interface{}{
		&model.User{},
		&model.Challenge{},
		&model.Team{},
		&relations.UserTeam{},
		&model.Submission{},
	}
	for _, v := range dbs {
		err := db.Sync2(v)
		if err != nil {
			return err
		}
	}
	return nil
}

func InitAdmin(db *xorm.Engine) {
	existAdminUser, _ := db.Table("user").Where("username = ?", "admin").Exist()
	if !existAdminUser {
		utils.Logger.Warn("超级管理员账户不存在，即将创建")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("admin"), bcrypt.DefaultCost)
		_, err := db.Table("user").Insert(model.User{
			UserId:   uuid.NewString(),
			Username: "admin",
			Name:     "超级管理员",
			Role:     0,
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		})
		if err != nil {
			utils.Logger.Error("超级管理员账户创建失败")
			os.Exit(1)
			return
		}
		utils.Logger.Infof("超级管理员账户创建成功")
	}
}
