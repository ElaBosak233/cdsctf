package controller

import (
	request2 "github.com/elabosak233/pgshub/internal/model/request"
	"github.com/elabosak233/pgshub/internal/model/response"
	"github.com/elabosak233/pgshub/internal/service"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog/log"
	"net/http"
)

type UserController struct {
	userService service.UserService
}

func NewUserController(appService service.AppService) *UserController {
	return &UserController{
		userService: appService.UserService,
	}
}

func (controller *UserController) Create(ctx *gin.Context) {
	createUserRequest := request2.CreateUserRequest{}
	err := ctx.ShouldBindJSON(&createUserRequest)
	utils.ErrorPanic(err)
	log.Info().Msg("create user")
	controller.userService.Create(createUserRequest)
	res := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, res)
}

func (controller *UserController) Update(ctx *gin.Context) {
	log.Info().Msg("update tags")
	updateUserRequest := request2.UpdateUserRequest{}
	err := ctx.ShouldBindJSON(&updateUserRequest)
	utils.ErrorPanic(err)
	id := ctx.Param("id")
	updateUserRequest.Id = id

	controller.userService.Update(updateUserRequest)

	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)
}

func (controller *UserController) Delete(ctx *gin.Context) {
	log.Info().Msg("delete tags")
	id := ctx.Param("id")
	controller.userService.Delete(id)

	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)
}

func (controller *UserController) FindById(ctx *gin.Context) {
	log.Info().Msg("findbyid user")
	id := ctx.Param("id")
	tagResponse := controller.userService.FindById(id)

	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   tagResponse,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)
}

func (controller *UserController) FindByUsername(ctx *gin.Context) {
	log.Info().Msg("findbyusername user")
	username := ctx.Param("username")
	tagResponse := controller.userService.FindByUsername(username)
	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   tagResponse,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)
}

func (controller *UserController) FindAll(ctx *gin.Context) {
	log.Info().Msg("findAll tags")
	tagResponse := controller.userService.FindAll()
	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   tagResponse,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)

}
