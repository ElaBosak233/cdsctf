package main

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/router"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
	"strconv"
)

func main() {
	// 打印欢迎字符
	utils.Welcome()
	// 初始化 Logger
	utils.InitLogger()
	// 初始化配置文件
	utils.LoadConfig()
	// 创建数据库连接
	db := utils.DatabaseConnection()
	_ = db.Sync(
		&data.User{},
		&data.Group{},
		&data.Challenge{},
	)

	debug, _ := strconv.ParseBool(os.Getenv("DEBUG"))
	if debug {
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	r := gin.Default()

	// Repositories
	appRepository := repository.AppRepository{
		GroupRepository:     repository.NewGroupRepositoryImpl(db),
		UserRepository:      repository.NewUserRepositoryImpl(db),
		ChallengeRepository: repository.NewChallengeRepositoryImpl(db),
	}

	// Services
	appService := service.AppService{
		UserService:      service.NewUserServiceImpl(appRepository),
		GroupService:     service.NewGroupServiceImpl(appRepository),
		ChallengeService: service.NewChallengeServiceImpl(appRepository),
	}

	// Controllers
	router.NewRouters(
		r,
		controller.NewUserController(appService),
		controller.NewGroupController(appService),
		controller.NewChallengeController(appService),
	)

	s := &http.Server{
		Addr:    utils.Cfg.Server.Host + ":" + strconv.Itoa(utils.Cfg.Server.Port),
		Handler: r,
	}
	utils.Logger.Infof("PgsHub Core 服务已启动，访问地址 %s:%d", utils.Cfg.Server.Host, utils.Cfg.Server.Port)
	_ = s.ListenAndServe()
}
