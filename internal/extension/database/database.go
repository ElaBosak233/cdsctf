package database

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/extension/config"
	"github.com/elabosak233/cloudsdale/internal/extension/logger/adapter"
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
	initAdmin()
	initCategory()
	selfCheck()
}

func Db() *gorm.DB {
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
		&model.Category{},
		&model.Challenge{},
		&model.Team{},
		&model.UserTeam{},
		&model.Submission{},
		&model.Nat{},
		&model.Hint{},
		&model.Pod{},
		&model.Game{},
		&model.GameChallenge{},
		&model.GameTeam{},
		&model.Flag{},
		&model.FlagGen{},
		&model.Port{},
		&model.Nat{},
		&model.Env{},
		&model.Notice{},
	)
	if err != nil {
		zap.L().Fatal("Database sync failed.", zap.Error(err))
	}
}

func selfCheck() {
	// 对于 pods 中的所有数据，若 removed_at 大于当前时间，则强制赋值为现在的时间，以免后续程序错误判断
	db.Model(&model.Pod{}).Where("removed_at > ?", time.Now().UnixMilli()).Update("removed_at", time.Now().UnixMilli())
}

func initAdmin() {
	var count int64
	db.Model(&model.User{}).Where("username = ?", "admin").Count(&count)
	if count == 0 {
		zap.L().Warn("Administrator account does not exist, will be created soon.")
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte("123456"), bcrypt.DefaultCost)
		admin := model.User{
			Username: "admin",
			Nickname: "Administrator",
			Group:    "admin",
			Password: string(hashedPassword),
			Email:    "admin@admin.com",
		}
		err := db.Create(&admin).Error
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
		zap.L().Warn("Categories do not exist, will be created soon.")
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
				Icon:        "language",
			},
			{
				Name:        "pwn",
				Description: "pwn",
				Color:       "#673AB7",
				Icon:        "function",
			},
			{
				Name:        "crypto",
				Description: "crypto",
				Color:       "#607D8B",
				Icon:        "tag",
			},
			{
				Name:        "reverse",
				Description: "reverse",
				Color:       "#6D4C41",
				Icon:        "keyboard_double_arrow_left",
			},
		}
		err := db.Create(&defaultCategories).Error
		if err != nil {
			zap.L().Fatal("Category initialization failed.", zap.Error(err))
			return
		}
	}
}