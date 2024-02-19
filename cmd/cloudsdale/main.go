package main

import "github.com/elabosak233/cloudsdale/internal/app"

// @title Cloudsdale API
// @version 1.0
// @description Hack for fun not for profit.
// @securityDefinitions.api_key	ApiKeyAuth
// @in header
// @name Authorization
// @BasePath /api
func main() {
	app.Run()
}
