package utils

import (
	_ "modernc.org/sqlite"
	"xorm.io/xorm"
)

const (
	host     = "localhost"
	port     = 5432
	user     = "postgres"
	password = "ela"
	dbName   = "test"
)

func DatabaseConnection() *xorm.Engine {
	db, err := xorm.NewEngine("sqlite", "sqlite.db")
	ErrorPanic(err)
	return db
}
