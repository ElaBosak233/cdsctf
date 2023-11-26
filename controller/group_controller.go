package controller

import (
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
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

func (c *GroupController) Create(ctx *gin.Context) {
	utils.Logger.WithFields(logrus.Fields{
		"ClientIP": ctx.ClientIP(),
	}).Info("Group 数据表记录创建")
	createGroupRequest := model.Group{}
	err := ctx.ShouldBindJSON(&createGroupRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &createGroupRequest),
		})
	}
	err = c.groupService.Create(createGroupRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *GroupController) Update(ctx *gin.Context) {
	utils.Logger.WithFields(logrus.Fields{
		"ClientIP": ctx.ClientIP(),
	}).Info("Group 数据表记录更新")
	updateGroupRequest := struct {
		Id   string `validate:"required"`
		Name string `validate:"required,max=20,min=3" json:"name"`
	}{}
	err := ctx.ShouldBindJSON(&updateGroupRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &updateGroupRequest),
		})
	}
	err = c.groupService.Update(model.Group{
		GroupId: updateGroupRequest.Id,
		Name:    updateGroupRequest.Name,
	})
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *GroupController) Delete(ctx *gin.Context) {
	utils.Logger.WithFields(logrus.Fields{
		"ClientIP": ctx.ClientIP(),
	}).Info("Group 数据表记录删除")
	id := ctx.Param("id")
	err := c.groupService.Delete(id)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *GroupController) FindById(ctx *gin.Context) {
	utils.Logger.WithFields(logrus.Fields{
		"ClientIP": ctx.ClientIP(),
	}).Info("Group 数据表记录通过 Id 查询")
	id := ctx.Param("id")
	groupResponse, err := c.groupService.FindById(id)
	if err != nil || groupResponse.GroupId == "" {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": groupResponse,
	})
}

func (c *GroupController) FindAll(ctx *gin.Context) {
	utils.Logger.WithFields(logrus.Fields{
		"ClientIP": ctx.ClientIP(),
	}).Info("Group 数据表记录全查询")
	groupResponse, err := c.groupService.FindAll()
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": groupResponse,
	})
}
