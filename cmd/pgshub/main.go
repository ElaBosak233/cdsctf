package main

import (
	"github.com/elabosak233/pgshub/config"
	"github.com/elabosak233/pgshub/internal/controller"
	"github.com/elabosak233/pgshub/internal/model/data"
	"github.com/elabosak233/pgshub/internal/repository"
	"github.com/elabosak233/pgshub/internal/router"
	"github.com/elabosak233/pgshub/internal/service"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/go-playground/validator/v10"
	"net/http"
)

func main() {
	utils.InitLogger()
	utils.Welcome()
	db := config.DatabaseConnection()
	validate := validator.New()

	err := db.Sync2(
		&data.User{},
		&data.Group{},
	)
	utils.ErrorPanic(err)

	// Repositories
	appRepository := repository.AppRepository{
		GroupRepository: repository.NewGroupRepositoryImpl(db),
		UserRepository:  repository.NewUserRepositoryImpl(db),
	}

	// Services
	appService := service.AppService{
		UserService:  service.NewUserServiceImpl(appRepository, validate),
		GroupService: service.NewGroupServiceImpl(appRepository, validate),
	}

	// Controllers
	routes := router.NewRouters(
		controller.NewUserController(appService),
		controller.NewGroupController(appService),
	)

	server := &http.Server{
		Addr:    ":8888",
		Handler: routes,
	}

	err = server.ListenAndServe()
	utils.ErrorPanic(err)
}
