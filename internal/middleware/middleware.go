package middleware

import "github.com/elabosak233/cloudsdale/internal/service"

type Middleware struct {
	FrontendMiddleware IFrontendMiddleware
	CasbinMiddleware   ICasbinMiddleware
}

func InitMiddleware(appService *service.Service) *Middleware {
	return &Middleware{
		FrontendMiddleware: NewFrontendMiddleware(),
		CasbinMiddleware:   NewCasbinMiddleware(appService),
	}
}
