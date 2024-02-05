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
	"github.com/elabosak233/pgshub/utils/logger"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"go.uber.org/zap"
	"net/http"
	"os"
)

var (
	CommitId = ""
	BuildAt  = ""
)

func init() {
	data, _ := assets.ReadStaticFile("banner.txt")
	banner := string(data)
	fmt.Printf("\n%s\n", banner)
	fmt.Printf("\n%s %s\n", color.InRed("WARNING"), color.InWhiteOverRed("PgsHub is still in development."))
	fmt.Printf("%s %s\n", color.InRed("WARNING"), color.InWhiteOverRed("All features are not guaranteed to work."))
	fmt.Printf("\n%s %s\n", color.InBold("Commit ID:"), color.InBold(CommitId))
	fmt.Printf("%s %s\n", color.InBold("Build At:"), color.InBold(BuildAt))
	fmt.Printf("%s %s\n\n", color.InBold("Issues:"), color.InBold("https://github.com/elabosak233/PgsHub/issues"))
}

// @title PgsHub Backend API
// @version 1.0
func main() {
	logger.InitLogger()
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
	r := gin.New()

	r.Use(logger.GinLogger(), logger.GinRecovery(true))

	// Cors 配置
	cor := cors.DefaultConfig()
	cor.AllowOrigins = viper.GetStringSlice("server.cors.allow_origins")
	cor.AllowMethods = viper.GetStringSlice("server.cors.allow_methods")
	cor.AllowHeaders = []string{"Origin", "Content-Length", "Content-Type", "Authorization"}
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
	zap.L().Info("The PgsHub service is launching! Enjoy your hacking challenges!")
	zap.L().Info(fmt.Sprintf("Here's the address! %s:%d", viper.GetString("server.host"), viper.GetInt("server.port")))
	err := s.ListenAndServe()
	if err != nil {
		zap.L().Error("Err... It seems that the port for PgsHub is not available. Plz try again.")
	}
}
