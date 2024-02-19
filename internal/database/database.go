package database

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/logger/adapter"
	"github.com/elabosak233/cloudsdale/internal/model"
	"go.uber.org/zap"
	"golang.org/x/crypto/bcrypt"
	"gorm.io/driver/mysql"
	"gorm.io/driver/postgres"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
	"time"
)

var db *gorm.DB
var dbInfo string

func InitDatabase() {
	initDatabaseEngine()
	zap.L().Info(fmt.Sprintf("Database Connect Information: %s", dbInfo))
	db.Logger = adapter.NewGORMAdapter(zap.L())
	syncDatabase()
	initGroup()
	initAdmin()
	initCategory()
	selfCheck()
}

func GetDatabase() *gorm.DB {
	return db
}

func Debug() {
	db = db.Debug()
}

func initDatabaseEngine() {
	var err error
	switch config.AppCfg().Db.Provider {
	case "postgres":
		dbInfo = fmt.Sprintf(
			"host=%s port=%d user=%s password=%s dbname=%s sslmode=%s",
			config.AppCfg().Db.Postgres.Host,
			config.AppCfg().Db.Postgres.Port,
			config.AppCfg().Db.Postgres.Username,
			config.AppCfg().Db.Postgres.Password,
			config.AppCfg().Db.Postgres.Dbname,
			config.AppCfg().Db.Postgres.Sslmode,
		)
		db, err = gorm.Open(postgres.Open(dbInfo), &gorm.Config{})
	case "mysql":
		dbInfo = fmt.Sprintf(
			"%s:%s@tcp(%s:%d)/%s?charset=utf8mb4&parseTime=True&loc=Local",
			config.AppCfg().Db.MySQL.Username,
			config.AppCfg().Db.MySQL.Password,
			config.AppCfg().Db.MySQL.Host,
			config.AppCfg().Db.MySQL.Port,
			config.AppCfg().Db.MySQL.Dbname,
		)
		db, err = gorm.Open(mysql.Open(dbInfo), &gorm.Config{})
	case "sqlite3":
		dbInfo = config.AppCfg().Db.SQLite3.Filename
		db, err = gorm.Open(sqlite.Open(dbInfo), &gorm.Config{})
	}
	if err != nil {
		zap.L().Fatal("Database connection failed.", zap.Error(err))
	}
}

func syncDatabase() {
	err := db.AutoMigrate(
		&model.User{},
		&model.Group{},
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
	)
	if err != nil {
		zap.L().Fatal("Database sync failed.", zap.Error(err))
	}
}

func selfCheck() {
	// 对于 pods 中的所有数据，若 removed_at 大于当前时间，则强制赋值为现在的时间，以免后续程序错误判断
	db.Model(&model.Pod{}).Where("removed_at > ?", time.Now().Unix()).Update("removed_at", time.Now().Unix())
}

func initAdmin() {
	var count int64
	db.Model(&model.User{}).Where("username = ?", "admin").Count(&count)
	if count == 0 {
		zap.L().Warn("Administrator account does not exist, will be created soon.")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("123456"), bcrypt.DefaultCost)
		err := db.Create(&model.User{
			Username: "admin",
			Nickname: "Administrator",
			GroupID:  1,
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		}).Error
		if err != nil {
			zap.L().Fatal("Super administrator account creation failed.", zap.Error(err))
			return
		}
		zap.L().Info("Super administrator account created successfully.")
	}
}

func initCategory() {
	var count int64
	db.Model(&model.Category{}).Count(&count)
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
		err := db.Create(&defaultCategories).Error
		if err != nil {
			zap.L().Fatal("Category initialization failed.", zap.Error(err))
			return
		}
	}
}

func initGroup() {
	var count int64
	db.Model(&model.Group{}).Count(&count)
	if count == 0 {
		zap.L().Warn("Groups do not exist, will be created soon.")
		defaultGroups := []model.Group{
			{
				Name:        "admin",
				Description: "The administrator has the highest authority.",
				Level:       1,
			},
			{
				Name:        "monitor",
				Description: "The monitor has the authority to control the games.",
				Level:       2,
			},
			{
				Name:        "user",
				Description: "The user is the default role.",
				Level:       3,
			},
			{
				Name:        "banned",
				Description: "The banned user has no authority.",
				Level:       5,
			},
		}
		err := db.Create(&defaultGroups).Error
		if err != nil {
			zap.L().Fatal("Groups initialization failed.", zap.Error(err))
			return
		}
		zap.L().Info("Groups created successfully.")
	}
}
