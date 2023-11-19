package controller

import (
	req "github.com/elabosak233/pgshub/model/request/account"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
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

func (c *UserController) Login(ctx *gin.Context) {
	userLoginRequest := req.UserLoginRequest{}
	if ctx.ShouldBindJSON(&userLoginRequest) != nil {
		utils.FormatErrorResponse(ctx)
		return
	}
	user := c.userService.FindById(userLoginRequest.Id)
	utils.Logger.WithFields(logrus.Fields{
		"Username": user.Username,
		"UserId":   userLoginRequest.Id,
		"ClientIP": ctx.ClientIP(),
	}).Info("登录")
	if !c.userService.VerifyPasswordById(userLoginRequest.Id, userLoginRequest.Password) {
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "用户名或密码错误",
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"token": c.userService.GetJwtTokenById(userLoginRequest.Id),
	})
}

func (c *UserController) Logout(ctx *gin.Context) {
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserController) Create(ctx *gin.Context) {
	createUserRequest := req.CreateUserRequest{}
	if ctx.ShouldBindJSON(&createUserRequest) != nil {
		utils.FormatErrorResponse(ctx)
		return
	}
	err := c.userService.Create(createUserRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "创建失败",
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserController) Update(ctx *gin.Context) {
	updateUserRequest := req.UpdateUserRequest{}
	if ctx.ShouldBindJSON(&updateUserRequest) != nil {
		utils.FormatErrorResponse(ctx)
		return
	}
	id := ctx.Param("id")
	updateUserRequest.Id = id
	c.userService.Update(updateUserRequest)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserController) Delete(ctx *gin.Context) {
	id := ctx.Param("id")
	c.userService.Delete(id)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserController) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	userResponse := c.userService.FindById(id)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}

func (c *UserController) FindByUsername(ctx *gin.Context) {
	username := ctx.Param("username")
	userResponse := c.userService.FindByUsername(username)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}

func (c *UserController) FindAll(ctx *gin.Context) {
	userResponse := c.userService.FindAll()
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}
