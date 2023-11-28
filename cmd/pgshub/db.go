package main

import (
	"fmt"
	model "github.com/elabosak233/pgshub/model/data"
	modelm2m "github.com/elabosak233/pgshub/model/data/m2m"
	"github.com/elabosak233/pgshub/utils"
	_ "github.com/go-sql-driver/mysql"
	"github.com/google/uuid"
	"github.com/spf13/viper"
	"golang.org/x/crypto/bcrypt"
	"os"
	"xorm.io/xorm"
)

func DatabaseConnection() *xorm.Engine {
	host := viper.GetString("MySql.Host")
	port := viper.GetInt("MySql.Port")
	user := viper.GetString("MySql.Username")
	password := viper.GetString("MySql.Password")
	dbName := viper.GetString("MySql.DbName")

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
	err := db.Sync2(
		&model.User{},
	)
	err = db.Sync2(
		&model.Group{},
	)
	err = db.Sync2(
		&modelm2m.UserGroup{},
	)
	err = db.Sync2(
		&model.Challenge{},
	)
	return err
}

func InitAdmin(db *xorm.Engine) {
	existAdminUser, _ := db.Table("user").Where("username = ?", "admin").Exist()
	if !existAdminUser {
		utils.Logger.Warn("管理员账户不存在，即将创建")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("admin"), bcrypt.DefaultCost)
		_, err := db.Table("user").Insert(model.User{
			UserId:   uuid.NewString(),
			Username: "admin",
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		})
		if err != nil {
			utils.Logger.Error("管理员账户创建失败")
			os.Exit(1)
			return
		}
		utils.Logger.Infof("管理员账户创建成功")
	}
	existAdminGroup, _ := db.Table("group").Where("name = ?", "admin").Exist()
	if !existAdminGroup {
		utils.Logger.Warn("管理员用户组不存在，即将创建")
		_, err := db.Table("group").Insert(model.Group{
			GroupId: uuid.NewString(),
			Name:    "admin",
		})
		if err != nil {
			utils.Logger.Error("管理员用户组创建失败")
			os.Exit(1)
			return
		}
		utils.Logger.Infof("管理员用户组创建成功")
	}
	adminUser := model.User{}
	adminGroup := model.Group{}
	_, _ = db.Table("user").Where("username = ?", "admin").Get(&adminUser)
	_, _ = db.Table("group").Where("name = ?", "admin").Get(&adminGroup)
	existAdminUserGroup, _ := db.Table("user_group").Exist(&modelm2m.UserGroup{
		UserId:  adminUser.UserId,
		GroupId: adminGroup.GroupId,
	})
	if !existAdminUserGroup {
		utils.Logger.Warn("管理员用户与用户组关系不存在，即将创建")
		_, err := db.Table("user_group").Insert(modelm2m.UserGroup{
			UserId:  adminUser.UserId,
			GroupId: adminGroup.GroupId,
		})
		if err != nil {
			utils.Logger.Error("管理员用户与用户组关系创建失败")
			os.Exit(1)
			return
		}
		utils.Logger.Infof("管理员用户与用户组关系创建成功")
	}
}
