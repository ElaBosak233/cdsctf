package main

import (
	"github.com/elabosak233/pgshub/cmd/pgshub/initialize"
	"github.com/elabosak233/pgshub/containers/providers"
	_ "github.com/elabosak233/pgshub/docs"
	"github.com/elabosak233/pgshub/routers"
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
func main() {
	Welcome()
	utils.InitLogger()
	utils.LoadConfig()
	db := initialize.GetDatabaseConnection()

	if viper.GetString("container.provider") == "docker" {
		providers.NewDockerProvider()
	}

	// Debug 模式
	debug, _ := strconv.ParseBool(os.Getenv("DEBUG"))
	if debug {
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
	appRepository := initialize.Repositories(db)
	appService := initialize.Services(appRepository)
	appMiddleware := initialize.Middlewares(appService)
	appController := initialize.Controllers(appService)
	routers.NewRouters(r.Group("/api"), appController, appMiddleware)

	// Swagger 文档
	r.GET("/docs/*any", ginSwagger.WrapHandler(swaggerFiles.NewHandler()))

	// 前端资源
	r.Use(appMiddleware.FrontendMiddleware.Frontend("/", "./dist"))

	s := &http.Server{
		Addr:    viper.GetString("server.host") + ":" + viper.GetString("server.port"),
		Handler: r,
	}
	utils.Logger.Infof("PgsHub 已启动，访问地址 %s:%d", viper.GetString("server.host"), viper.GetInt("server.port"))
	_ = s.ListenAndServe()
}
