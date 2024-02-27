package middleware

import "github.com/elabosak233/cloudsdale/internal/service"

type Middleware struct {
	AuthMiddleware     IAuthMiddleware
	FrontendMiddleware IFrontendMiddleware
	CasbinMiddleware   ICasbinMiddleware
}

func InitMiddleware(appService *service.Service) *Middleware {
	return &Middleware{
		AuthMiddleware:     NewAuthMiddleware(appService),
		FrontendMiddleware: NewFrontendMiddleware(),
		CasbinMiddleware:   NewCasbinMiddleware(appService),
	}
}
