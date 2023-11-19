package controller

import (
	req "github.com/elabosak233/pgshub/model/request/account"
	"github.com/elabosak233/pgshub/model/response"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"github.com/rs/zerolog/log"
	"net/http"
)

type GroupController struct {
	groupService service.GroupService
}

func NewGroupController(appService service.AppService) *GroupController {
	return &GroupController{
		groupService: appService.GroupService,
	}
}

func (controller *GroupController) Create(ctx *gin.Context) {
	createGroupRequest := req.CreateGroupRequest{}
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
	updateGroupRequest := req.UpdateGroupRequest{}
	err := ctx.ShouldBindJSON(&updateGroupRequest)
	utils.ErrorPanic(err)
	id := ctx.Param("id")
	updateGroupRequest.Id = id
	controller.groupService.Update(updateGroupRequest)
	res := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, res)
}

func (controller *GroupController) Delete(ctx *gin.Context) {
	log.Info().Msg("delete tags")
	id := ctx.Param("id")
	controller.groupService.Delete(id)
	res := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   nil,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, res)
}

func (controller *GroupController) FindById(ctx *gin.Context) {
	log.Info().Msg("findbyid tags")
	id := ctx.Param("id")
	tagResponse := controller.groupService.FindById(id)

	res := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   tagResponse,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, res)
}

func (controller *GroupController) FindAll(ctx *gin.Context) {
	log.Info().Msg("findAll tags")
	groupResponse := controller.groupService.FindAll()
	res := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
		Data:   groupResponse,
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, res)

}

func (controller *GroupController) AddUserToGroup(ctx *gin.Context) {
	addUserToGroupRequest := req.AddUserToGroupRequest{}
	if ctx.ShouldBindJSON(&addUserToGroupRequest) != nil {
		utils.FormatErrorResponse(ctx)
		return
	}
	controller.groupService.AddUserToGroup(addUserToGroupRequest)
	res := response.Response{
		Code:   http.StatusOK,
		Status: "Ok",
	}
	ctx.JSON(http.StatusOK, res)
}
