package main

import (
	controller2 "github.com/elabosak233/pgshub/controller"
	"github.com/elabosak233/pgshub/model/data"
	repository2 "github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/router"
	service2 "github.com/elabosak233/pgshub/service"
	utils2 "github.com/elabosak233/pgshub/utils"
	"net/http"
	"strconv"
)

func main() {
	// 初始化 Logger
	utils2.InitLogger()
	// 打印欢迎字符
	utils2.Welcome()
	// 初始化配置文件
	config, _ := utils2.LoadConfig()
	// 创建数据库连接
	db := utils2.DatabaseConnection()
	_ = db.Sync2(
		&data.User{},
		&data.Group{},
	)

	// Repositories
	appRepository := repository2.AppRepository{
		GroupRepository: repository2.NewGroupRepositoryImpl(db),
		UserRepository:  repository2.NewUserRepositoryImpl(db),
	}

	// Services
	appService := service2.AppService{
		UserService:  service2.NewUserServiceImpl(appRepository),
		GroupService: service2.NewGroupServiceImpl(appRepository),
	}

	// Controllers
	routes := router.NewRouters(
		controller2.NewUserController(appService),
		controller2.NewGroupController(appService),
	)

	server := &http.Server{
		Addr:    ":" + strconv.Itoa(config.Server.Port),
		Handler: routes,
	}

	_ = server.ListenAndServe()
}
