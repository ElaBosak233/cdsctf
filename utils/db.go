package utils

import (
	_ "modernc.org/sqlite"
	"xorm.io/xorm"
)

func DatabaseConnection() *xorm.Engine {
	//host := Cfg.Database.Host
	//post := Cfg.Database.Port
	//user := Cfg.Database.Username
	//password := Cfg.Database.Password
	//dbName := Cfg.Database.DbName
	db, err := xorm.NewEngine("sqlite", "sqlite.db")
	if err != nil {
		Logger.Error("数据库连接失败")
		return nil
	}
	return db
}
