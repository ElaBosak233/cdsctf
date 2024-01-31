package main

import (
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/elabosak233/pgshub/containers/providers"
	"github.com/elabosak233/pgshub/controllers"
	_ "github.com/elabosak233/pgshub/docs"
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/routers"
	"github.com/elabosak233/pgshub/services"
	"github.com/elabosak233/pgshub/utils/assets"
	"github.com/elabosak233/pgshub/utils/config"
	"github.com/elabosak233/pgshub/utils/convertor"
	"github.com/elabosak233/pgshub/utils/database"
	log "github.com/elabosak233/pgshub/utils/logger"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"net/http"
	"os"
)

func init() {
	data, _ := assets.ReadStaticFile("banner.txt")
	fmt.Println(color.Ize(color.CyanBackground, string(data)))
}

// @title PgsHub Backend API
// @version 1.0
func main() {
	config.InitConfig()
	database.InitDatabase()

	if viper.GetString("container.provider") == "docker" {
		providers.NewDockerProvider()
	}

	// Debug 模式
	if convertor.ToBoolD(os.Getenv("DEBUG"), false) {
		database.GetDatabase().ShowSQL(true)
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	r := gin.Default()

	// Cors 配置
	cor := cors.DefaultConfig()
	cor.AllowOrigins = viper.GetStringSlice("server.cors.allow_origins")
	cor.AllowMethods = viper.GetStringSlice("server.cors.allow_methods")
	cor.AllowHeaders = []string{"Origin", "Content-Length", "Content-Type", "PgsToken"}
	cor.AllowCredentials = true
	r.Use(cors.New(cor))

	// 依赖注入
	appRepository := repositories.InitRepositories(database.GetDatabase())
	appService := services.InitServices(appRepository)
	appMiddleware := middlewares.InitMiddlewares(appService)
	appController := controllers.InitControllers(appService)
	routers.NewRouters(r.Group("/api"), appController, appMiddleware)

	// Swagger 文档
	r.GET("/docs/*any", ginSwagger.WrapHandler(swaggerFiles.NewHandler()))

	// 前端资源
	r.Use(appMiddleware.FrontendMiddleware.Frontend("/", "./dist"))

	s := &http.Server{
		Addr:    viper.GetString("server.host") + ":" + viper.GetString("server.port"),
		Handler: r,
	}
	log.Infof("PgsHub 已启动，访问地址 %s:%d", viper.GetString("server.host"), viper.GetInt("server.port"))
	_ = s.ListenAndServe()
}
