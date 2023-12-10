package main

import (
	_ "github.com/elabosak233/pgshub/docs"
	"github.com/elabosak233/pgshub/middleware"
	"github.com/elabosak233/pgshub/router"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"net/http"
	"os"
	"strconv"
)

// @title PgsHub Backend API
// @version 1.0
// @description 没有其他东西啦，仅仅是所有的后端接口，不要乱用哦
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
	cor.AllowOrigins = []string{"*"}
	cor.AllowMethods = []string{"GET", "POST", "PUT", "DELETE"}
	r.Use(cors.New(cor))

	appRepository := InitRepositories(db)
	appService := InitServices(appRepository)
	appController := InitControllers(appService)
	api := r.Group("/api")
	router.NewRouters(api, appController)

	r.GET("/docs/*any", ginSwagger.WrapHandler(swaggerFiles.NewHandler()))

	r.Use(middleware.FrontendMiddleware("/", "./dist"))

	s := &http.Server{
		Addr:    viper.GetString("Server.Host") + ":" + viper.GetString("Server.Port"),
		Handler: r,
	}
	utils.Logger.Infof("PgsHub Core 服务已启动，访问地址 %s:%d", viper.GetString("Server.Host"), viper.GetInt("Server.Port"))
	_ = s.ListenAndServe()
}
