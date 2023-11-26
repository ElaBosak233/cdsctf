package main

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/elabosak233/pgshub/router"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
	"strconv"
)

func main() {
	Welcome()
	utils.InitLogger()
	utils.LoadConfig()
	db := DatabaseConnection()

	debug, _ := strconv.ParseBool(os.Getenv("DEBUG"))
	if debug {
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	r := gin.Default()

	cor := cors.DefaultConfig()
	cor.AllowOrigins = []string{"http://localhost:3000"}
	cor.AllowMethods = []string{"GET", "POST", "PUT", "DELETE"}
	r.Use(cors.New(cor))

	appRepository := InitRepositories(db)
	appService := InitServices(appRepository)
	router.NewRouters(
		r,
		controller.NewUserController(appService),
		controller.NewGroupController(appService),
		controller.NewChallengeController(appService),
		controller.NewUserGroupController(appService),
	)

	s := &http.Server{
		Addr:    utils.Config.Server.Host + ":" + strconv.Itoa(utils.Config.Server.Port),
		Handler: r,
	}
	utils.Logger.Infof("PgsHub Core 服务已启动，访问地址 %s:%d", utils.Config.Server.Host, utils.Config.Server.Port)
	_ = s.ListenAndServe()
}
