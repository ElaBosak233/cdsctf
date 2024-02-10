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
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"go.uber.org/zap"
	"net/http"
	"os"
	"strconv"
)

var (
	GitCommitID = "N/A"
	AppBuildAt  = "N/A"
)

func init() {
	data, _ := assets.ReadStaticFile("banner.txt")
	banner := string(data)
	fmt.Printf("\n%s\n", banner)
	fmt.Printf("%s %s\n", color.InBold("Commit IDs:"), color.InBold(GitCommitID))
	fmt.Printf("%s %s\n", color.InBold("Build At:"), color.InBold(AppBuildAt))
	fmt.Printf("%s %s\n\n", color.InBold("Issues:"), color.InBold("https://github.com/elabosak233/PgsHub/issues"))
}

// @title PgsHub Backend API
// @version 1.0
func main() {
	logger.InitLogger()
	config.InitConfig()
	database.InitDatabase()

	switch config.Cfg().Container.Provider {
	case "docker":
		providers.NewDockerProvider()
	}

	// Debug mode
	if convertor.ToBoolD(os.Getenv("DEBUG"), false) {
		database.GetDatabase().ShowSQL(true)
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	r := gin.New()

	r.Use(logger.GinLogger(), logger.GinRecovery(true))

	// Cors configurations
	cor := cors.DefaultConfig()
	cor.AllowOrigins = config.Cfg().Server.CORS.AllowOrigins
	cor.AllowMethods = config.Cfg().Server.CORS.AllowMethods
	cor.AllowHeaders = []string{"Origin", "Content-Length", "Content-Type", "Authorization"}
	cor.AllowCredentials = true
	r.Use(cors.New(cor))

	// Dependencies injection
	appRepository := repositories.InitRepositories(database.GetDatabase())
	appService := services.InitServices(appRepository)
	appMiddleware := middlewares.InitMiddlewares(appService)
	appController := controllers.InitControllers(appService)
	routers.NewRouters(r.Group("/api"), appController, appMiddleware)

	// Swagger docs
	r.GET("/docs/*any", ginSwagger.WrapHandler(swaggerFiles.NewHandler()))

	// Frontend resources
	r.Use(appMiddleware.FrontendMiddleware.Frontend("/", "./dist"))

	s := &http.Server{
		Addr:    config.Cfg().Server.Host + ":" + strconv.Itoa(config.Cfg().Server.Port),
		Handler: r,
	}
	zap.L().Info("The PgsHub service is launching! Enjoy your hacking challenges!")
	zap.L().Info(fmt.Sprintf("Here's the address! %s:%d", config.Cfg().Server.Host, config.Cfg().Server.Port))
	err := s.ListenAndServe()
	if err != nil {
		zap.L().Error("Err... It seems that the port for PgsHub is not available. Plz try again.")
	}
}
