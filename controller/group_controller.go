package controller

import (
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/model/response"
	service2 "github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog/log"
	"net/http"
)

type GroupController struct {
	groupService service2.GroupService
}

func NewGroupController(appService service2.AppService) *GroupController {
	return &GroupController{
		groupService: appService.GroupService,
	}
}

func (controller *GroupController) Create(ctx *gin.Context) {
	createGroupRequest := request.CreateGroupRequest{}
	err := ctx.ShouldBindJSON(&createGroupRequest)
	utils.ErrorPanic(err)
	log.Info().Msg("create group")
	controller.groupService.Create(createGroupRequest)
	res := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, res)
}

func (controller *GroupController) Update(ctx *gin.Context) {
	log.Info().Msg("update tags")
	updateGroupRequest := request.UpdateGroupRequest{}
	err := ctx.ShouldBindJSON(&updateGroupRequest)
	utils.ErrorPanic(err)
	id := ctx.Param("id")
	updateGroupRequest.Id = id

	controller.groupService.Update(updateGroupRequest)

	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)
}

func (controller *GroupController) Delete(ctx *gin.Context) {
	log.Info().Msg("delete tags")
	id := ctx.Param("id")
	controller.groupService.Delete(id)

	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)
}

func (controller *GroupController) FindById(ctx *gin.Context) {
	log.Info().Msg("findbyid tags")
	id := ctx.Param("id")
	tagResponse := controller.groupService.FindById(id)

	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   tagResponse,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)
}

func (controller *GroupController) FindAll(ctx *gin.Context) {
	log.Info().Msg("findAll tags")
	groupResponse := controller.groupService.FindAll()
	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   groupResponse,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, webResponse)

}

func (controller *GroupController) AddUserToGroup(ctx *gin.Context) {
	groupId := ctx.Param("id")
	addUserToGroupRequest := request.AddUserToGroupRequest{}
	err := ctx.ShouldBindJSON(&addUserToGroupRequest)
	utils.ErrorPanic(err)
	controller.groupService.AddUserToGroup(groupId, addUserToGroupRequest)
	webResponse := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
	}
	ctx.JSON(http.StatusOK, webResponse)
}
