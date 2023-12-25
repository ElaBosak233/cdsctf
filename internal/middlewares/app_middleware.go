package middlewares

type AppMiddleware struct {
	AuthMiddleware     AuthMiddleware
	FrontendMiddleware FrontendMiddleware
}
