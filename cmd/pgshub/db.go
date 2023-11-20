package main

import (
	"fmt"
	model "github.com/elabosak233/pgshub/model/data"
	modelm2m "github.com/elabosak233/pgshub/model/data/m2m"
	"github.com/elabosak233/pgshub/utils"
	_ "github.com/go-sql-driver/mysql"
	"github.com/google/uuid"
	_ "github.com/lib/pq"
	"golang.org/x/crypto/bcrypt"
	"xorm.io/xorm"
)

func DatabaseConnection() *xorm.Engine {
	host := utils.Config.MySql.Host
	port := utils.Config.MySql.Port
	user := utils.Config.MySql.Username
	password := utils.Config.MySql.Password
	dbName := utils.Config.MySql.DbName

	dbInfo := fmt.Sprintf(
		"%s:%s@tcp(%s:%d)/%s?charset=utf8mb4",
		user,
		password,
		host,
		port,
		dbName,
	)

	utils.Logger.Info("数据库连接信息 " + dbInfo)

	db, err := xorm.NewEngine("mysql", dbInfo)
	if err != nil {
		utils.Logger.Error("数据库连接失败")
		return nil
	}
	SyncDatabase(db)
	InitAdmin(db)
	return db
}

func SyncDatabase(db *xorm.Engine) {
	_ = db.Sync2(
		&model.User{},
	)
	_ = db.Sync2(
		&model.Group{},
	)
	_ = db.Sync2(
		&modelm2m.UserGroup{},
	)
	_ = db.Sync2(
		&model.Challenge{},
	)
}

func InitAdmin(db *xorm.Engine) {
	existAdminUser, _ := db.Table("user").Where("username = ?", "admin").Exist()
	if !existAdminUser {
		utils.Logger.Warn("管理员账户不存在，即将创建")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("admin"), bcrypt.DefaultCost)
		_, err := db.Table("user").Insert(model.User{
			Id:       uuid.NewString(),
			Username: "admin",
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		})
		if err != nil {
			utils.Logger.Error("管理员账户创建失败")
			return
		}
		utils.Logger.Infof("管理员账户创建成功")
	}
	existAdminGroup, _ := db.Table("group").Where("name = ?", "admin").Exist()
	if !existAdminGroup {
		utils.Logger.Warn("管理员用户组不存在，即将创建")
		_, err := db.Table("group").Insert(model.Group{
			Id:          uuid.NewString(),
			Name:        "admin",
			Permissions: []string{"admin"},
		})
		if err != nil {
			utils.Logger.Error("管理员用户组创建失败")
			return
		}
		utils.Logger.Infof("管理员用户组创建成功")
	}
	adminUser := model.User{}
	adminGroup := model.Group{}
	_, _ = db.Table("user").Where("username = ?", "admin").Get(&adminUser)
	_, _ = db.Table("group").Where("name = ?", "admin").Get(&adminGroup)
	existAdminUserGroup, _ := db.Table("user_group").Exist(&modelm2m.UserGroup{
		UserId:  adminUser.Id,
		GroupId: adminGroup.Id,
	})
	if !existAdminUserGroup {
		utils.Logger.Warn("管理员用户与用户组关系不存在，即将创建")
		_, err := db.Table("user_group").Insert(modelm2m.UserGroup{
			UserId:  adminUser.Id,
			GroupId: adminGroup.Id,
		})
		if err != nil {
			utils.Logger.Error("管理员用户与用户组关系创建失败")
			return
		}
		utils.Logger.Infof("管理员用户与用户组关系创建成功")
	}
}
