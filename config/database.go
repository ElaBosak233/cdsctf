package config

import (
	"github.com/elabosak233/pgshub/internal/utils"
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
	//sqlInfo := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable", host, port, user, password, dbName)
	db, err := xorm.NewEngine("sqlite", "sqlite.db")
	utils.ErrorPanic(err)
	return db
}
