package controller

import (
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
	"net/http"
)

type UserControllerImpl struct {
	userService service.UserService
}

func NewUserControllerImpl(appService *service.AppService) UserController {
	return &UserControllerImpl{
		userService: appService.UserService,
	}
}

// Login
// @Summary 用户登录
// @Description 用户登录
// @Tags 用户
// @Accept json
// @Produce json
// @Param input body request.UserLoginRequest true "UserLoginRequest"
// @Router /api/user/login [post]
func (c *UserControllerImpl) Login(ctx *gin.Context) {
	userLoginRequest := request.UserLoginRequest{}
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

// VerifyToken
// @Summary Token 鉴定
// @Description Token 鉴定
// @Tags 用户
// @Produce json
// @Param token path string true "token"
// @Router /api/user/verifyToken/{token} [get]
func (c *UserControllerImpl) VerifyToken(ctx *gin.Context) {
	id, err := c.userService.GetIdByJwtToken(ctx.Param("token"))
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	}
	if id == "" {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "Token 无效",
		})
	} else {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"id":   id,
		})
	}
}

// Logout
// @Summary 用户登出
// @Description 用户登出
// @Tags 用户
// @Accept json
// @Produce json
// @Param input body request.UserLogoutRequest true "UserLogoutRequest"
// @Router /api/user/logout [post]
func (c *UserControllerImpl) Logout(ctx *gin.Context) {
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Register
// @Summary 用户注册
// @Description 用户注册
// @Tags 用户
// @Accept json
// @Produce json
// @Param input body request.UserRegisterRequest true "UserRegisterRequest"
// @Router /api/user/register [post]
func (c *UserControllerImpl) Register(ctx *gin.Context) {
	createUserRequest := request.UserRegisterRequest{}
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

func (c *UserControllerImpl) Create(ctx *gin.Context) {
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

func (c *UserControllerImpl) Update(ctx *gin.Context) {
	updateUserRequest := request.UserUpdateRequest{}
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

func (c *UserControllerImpl) Delete(ctx *gin.Context) {
	id := ctx.Param("id")
	c.userService.Delete(id)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

func (c *UserControllerImpl) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	userResponse, _ := c.userService.FindById(id)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}

func (c *UserControllerImpl) FindByUsername(ctx *gin.Context) {
	username := ctx.Param("username")
	userResponse, _ := c.userService.FindByUsername(username)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}

func (c *UserControllerImpl) FindAll(ctx *gin.Context) {
	userResponse, _ := c.userService.FindAll()
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}
