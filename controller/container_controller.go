package controller

import (
	"github.com/elabosak233/pgshub/service"
	"github.com/gin-gonic/gin"
)

type ContainerController struct {
}

func NewContainerController(appService service.AppService) *ContainerController {
	return &ContainerController{}
}

func (c *ContainerController) Create(ctx *gin.Context) {

}

func (c *ContainerController) Remove(ctx *gin.Context) {

}

func (c *ContainerController) Update(ctx *gin.Context) {}
