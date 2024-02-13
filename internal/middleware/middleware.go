package middleware

import "github.com/elabosak233/pgshub/internal/service"

type Middleware struct {
	AuthMiddleware     IAuthMiddleware
	FrontendMiddleware IFrontendMiddleware
}

func InitMiddleware(appService *service.Service) *Middleware {
	return &Middleware{
		AuthMiddleware:     NewAuthMiddleware(appService),
		FrontendMiddleware: NewFrontendMiddleware(),
	}
}
