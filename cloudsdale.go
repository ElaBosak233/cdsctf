package main

import "github.com/elabosak233/cloudsdale/internal/app"

// @title Cloudsdale
// @securityDefinitions.api_key	ApiKeyAuth
// @in header
// @name Authorization
// @BasePath /api
func main() {
	app.Run()
}
