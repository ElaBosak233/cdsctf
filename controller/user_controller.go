package controller

import (
	model "github.com/elabosak233/pgshub/model/data"
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
	userLoginRequest := struct {
		Username string `binding:"required" json:"username"`
		Password string `binding:"required" json:"password"`
	}{}
	err := ctx.ShouldBindJSON(&userLoginRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &userLoginRequest),
		})
		return
	}
	user, _ := c.userService.FindByUsername(userLoginRequest.Username)
	utils.Logger.WithFields(logrus.Fields{
		"Username": user.Username,
		"UserId":   user.Id,
		"ClientIP": ctx.ClientIP(),
	}).Info("登录")
	if !c.userService.VerifyPasswordByUsername(userLoginRequest.Username, userLoginRequest.Password) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "用户名或密码错误",
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"token": c.userService.GetJwtTokenById(user.Id),
	})
}

func (c *UserController) Logout(ctx *gin.Context) {
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserController) Register(ctx *gin.Context) {
	createUserRequest := model.User{}
	err := ctx.ShouldBindJSON(&createUserRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &createUserRequest),
		})
		return
	}
	err = c.userService.Create(model.User{
		Username: createUserRequest.Username,
		Email:    createUserRequest.Email,
		Password: createUserRequest.Password,
	})
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "用户名或邮箱重复",
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserController) Create(ctx *gin.Context) {
	createUserRequest := model.User{}
	err := ctx.ShouldBindJSON(&createUserRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &createUserRequest),
		})
		return
	}
	err = c.userService.Create(model.User{
		Username: createUserRequest.Username,
		Email:    createUserRequest.Email,
		Password: createUserRequest.Password,
	})
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "用户名或邮箱重复",
		})
		return
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserController) Update(ctx *gin.Context) {
	updateUserRequest := service.UserUpdateRequest{}
	err := ctx.ShouldBindJSON(&updateUserRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &updateUserRequest),
		})
		return
	}
	err = c.userService.Update(updateUserRequest)
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
	userResponse, _ := c.userService.FindById(id)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}

func (c *UserController) FindByUsername(ctx *gin.Context) {
	username := ctx.Param("username")
	userResponse, _ := c.userService.FindByUsername(username)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}

func (c *UserController) FindAll(ctx *gin.Context) {
	userResponse, _ := c.userService.FindAll()
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}
