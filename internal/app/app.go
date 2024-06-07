package app

import (
	"fmt"
	_ "github.com/elabosak233/cloudsdale/api"
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"github.com/elabosak233/cloudsdale/internal/app/db"
	"github.com/elabosak233/cloudsdale/internal/app/logger"
	"github.com/elabosak233/cloudsdale/internal/app/logger/adapter"
	"github.com/elabosak233/cloudsdale/internal/cache"
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/extension/casbin"
	"github.com/elabosak233/cloudsdale/internal/extension/container/provider"
	"github.com/elabosak233/cloudsdale/internal/files"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/router"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-contrib/cors"
	ginI18n "github.com/gin-contrib/i18n"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	v10 "github.com/go-playground/validator/v10"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
	"go.uber.org/zap"
	"golang.org/x/text/language"
	"html/template"
	"k8s.io/apimachinery/pkg/util/yaml"
	"net/http"
	"os"
)

func init() {
	data, _ := files.F().ReadFile("statics/banner.txt")
	banner := string(data)
	t, _ := template.New("cloudsdale").Parse(banner)
	_ = t.Execute(os.Stdout, struct {
		Version string
		Commit  string
	}{
		Version: utils.GitTag,
		Commit:  utils.GitCommitID,
	})
}

func Run() {
	// Initialize the application
	logger.InitLogger()
	config.InitConfig()
	db.InitDatabase()
	casbin.InitCasbin()
	provider.InitContainerProvider()
	cache.InitCache()

	// Debug mode
	isDebug := convertor.ToBoolD(os.Getenv("DEBUG"), false)
	if isDebug {
		db.Debug()
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	r := gin.New()

	if v, ok := binding.Validator.Engine().(*v10.Validate); ok {
		_ = v.RegisterValidation("ascii", validator.IsASCII)
	}

	r.Use(adapter.GinLogger(), adapter.GinRecovery(true))

	// I18n configurations
	r.Use(ginI18n.Localize(ginI18n.WithBundle(&ginI18n.BundleCfg{
		RootPath:         "./i18n",
		AcceptLanguage:   []language.Tag{language.English, language.SimplifiedChinese},
		DefaultLanguage:  language.English,
		UnmarshalFunc:    yaml.Unmarshal,
		FormatBundleFile: "yaml",
		Loader:           &ginI18n.EmbedLoader{FS: files.F()},
	})))

	// Cors configurations
	cor := cors.DefaultConfig()
	cor.AllowOrigins = config.AppCfg().Gin.CORS.AllowOrigins
	cor.AllowMethods = config.AppCfg().Gin.CORS.AllowMethods
	cor.AllowHeaders = []string{"Origin", "Content-Length", "Content-Type", "Authorization"}
	cor.AllowCredentials = true
	r.Use(cors.New(cor))

	r.OPTIONS("/*path", func(c *gin.Context) {
		c.Status(http.StatusOK)
	})

	// Initialize the application
	repository.InitRepository()
	service.InitService()
	controller.InitController()
	router.InitRouter(r.Group("/api", middleware.Casbin()))

	if isDebug {
		// Swagger docs
		r.GET("/docs/*any",
			ginSwagger.WrapHandler(
				swaggerFiles.NewHandler(),
				ginSwagger.PersistAuthorization(true),
			),
		)
	}

	// Frontend resources
	r.Use(middleware.Frontend("/"))

	srv := &http.Server{
		Addr: fmt.Sprintf(
			"%s:%d",
			config.AppCfg().Gin.Host,
			config.AppCfg().Gin.Port,
		),
		Handler: r,
	}
	zap.L().Info(fmt.Sprintf("Here's the address! %s:%d", config.AppCfg().Gin.Host, config.AppCfg().Gin.Port))
	zap.L().Info("The Cloudsdale service is running! Enjoy your hacking challenges!")
	err := srv.ListenAndServe()
	if err != nil {
		zap.L().Fatal("Err... It seems that the port for Cloudsdale is not available. Plz try again.")
	}
}
