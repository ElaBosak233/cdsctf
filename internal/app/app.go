package app

import (
	"fmt"
	"github.com/TwiN/go-color"
	_ "github.com/elabosak233/cloudsdale/docs"
	"github.com/elabosak233/cloudsdale/internal/assets"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/container/provider"
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/database"
	"github.com/elabosak233/cloudsdale/internal/global"
	"github.com/elabosak233/cloudsdale/internal/logger"
	"github.com/elabosak233/cloudsdale/internal/logger/adapter"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/router"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"go.uber.org/zap"
	"net/http"
	"os"
	"strconv"
)

func init() {
	data, _ := assets.ReadStaticFile("banner.txt")
	banner := string(data)
	fmt.Printf("\n%s\n", banner)
	fmt.Printf("%s %s\n", color.InBold("Commit IDs:"), color.InBold(global.GitCommitID))
	fmt.Printf("%s %s\n", color.InBold("Build At:"), color.InBold(global.BuildAt))
	fmt.Printf("%s %s\n\n", color.InBold("Issues:"), color.InBold("https://github.com/elabosak233/Cloudsdale/issues"))
}

func Run() {
	logger.InitLogger()
	config.InitConfig()
	assets.InitAssets()
	database.InitDatabase()

	switch config.AppCfg().Container.Provider {
	case "docker":
		provider.NewDockerProvider()
	case "k8s":
		provider.NewK8sProvider()
	default:
		zap.L().Fatal("Invalid container provider!")
	}

	// Debug mode
	if convertor.ToBoolD(os.Getenv("DEBUG"), false) {
		database.Debug()
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	r := gin.New()

	r.Use(adapter.GinLogger(), adapter.GinRecovery(true))

	// Cors configurations
	cor := cors.DefaultConfig()
	cor.AllowOrigins = config.AppCfg().Gin.CORS.AllowOrigins
	cor.AllowMethods = config.AppCfg().Gin.CORS.AllowMethods
	cor.AllowHeaders = []string{"Origin", "Content-Length", "Content-Type", "Authorization"}
	cor.AllowCredentials = true
	r.Use(cors.New(cor))

	// Dependencies injection
	appRepository := repository.InitRepository(database.GetDatabase())
	appService := service.InitService(appRepository)
	appMiddleware := middleware.InitMiddleware(appService)
	appController := controller.InitController(appService)
	router.NewRouter(r.Group("/api"), appController, appMiddleware)

	// Swagger docs
	r.GET("/docs/*any", ginSwagger.WrapHandler(swaggerFiles.NewHandler()))

	// Frontend resources
	r.Use(appMiddleware.FrontendMiddleware.Frontend("/"))

	s := &http.Server{
		Addr:    config.AppCfg().Gin.Host + ":" + strconv.Itoa(config.AppCfg().Gin.Port),
		Handler: r,
	}
	zap.L().Info("The Cloudsdale service is launching! Enjoy your hacking challenges!")
	zap.L().Info(fmt.Sprintf("Here's the address! %s:%d", config.AppCfg().Gin.Host, config.AppCfg().Gin.Port))
	err := s.ListenAndServe()
	if err != nil {
		zap.L().Fatal("Err... It seems that the port for Cloudsdale is not available. Plz try again.")
	}
}
