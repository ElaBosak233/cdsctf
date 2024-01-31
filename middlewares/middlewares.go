package middlewares

import "github.com/elabosak233/pgshub/services"

type Middlewares struct {
	AuthMiddleware     AuthMiddleware
	FrontendMiddleware FrontendMiddleware
}

func InitMiddlewares(appService *services.Services) *Middlewares {
	return &Middlewares{
		AuthMiddleware:     NewAuthMiddleware(appService),
		FrontendMiddleware: NewFrontendMiddleware(),
	}
}
