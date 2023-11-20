package controller

import (
	model "github.com/elabosak233/pgshub/model/data/m2m"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/service/m2m"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
	"net/http"
)

type UserGroupController struct {
	userGroupService m2m.UserGroupService
}

func NewUserGroupController(appService service.AppService) *UserGroupController {
	return &UserGroupController{
		userGroupService: appService.UserGroupService,
	}
}

func (c *UserGroupController) Create(ctx *gin.Context) {
	utils.Logger.Info("User_Group 关联表记录创建")
	userGroup := model.UserGroup{}
	err := ctx.ShouldBindJSON(&userGroup)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &userGroup),
		})
		return
	}
	err = c.userGroupService.Create(userGroup)
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

func (c *UserGroupController) Delete(ctx *gin.Context) {
	utils.Logger.WithFields(logrus.Fields{
		"ClientIP": ctx.ClientIP(),
	}).Info("User_Group 关联表记录删除")
	userGroup := model.UserGroup{}
	err := ctx.ShouldBindJSON(&userGroup)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &userGroup),
		})
		return
	}
	err = c.userGroupService.Delete(userGroup)
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
