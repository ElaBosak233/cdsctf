package initialize

import (
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/elabosak233/pgshub/services"
)

func Middlewares(appService *services.AppService) *middlewares.AppMiddleware {
	return &middlewares.AppMiddleware{
		AuthMiddleware:     middlewares.NewAuthMiddleware(appService),
		FrontendMiddleware: middlewares.NewFrontendMiddleware(),
	}
}
