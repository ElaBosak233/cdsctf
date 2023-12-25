package main

import (
	"github.com/elabosak233/pgshub/internal/middlewares"
	"github.com/elabosak233/pgshub/internal/middlewares/implements"
	"github.com/elabosak233/pgshub/internal/services"
)

func InitMiddlewares(appService *services.AppService) *middlewares.AppMiddleware {
	return &middlewares.AppMiddleware{
		AuthMiddleware:     implements.NewAuthMiddleware(appService),
		FrontendMiddleware: implements.NewFrontendMiddleware(),
	}
}
