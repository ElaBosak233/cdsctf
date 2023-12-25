package controllers

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
	"net/http"
)

type UserControllerImpl struct {
	userService services.UserService
}

func NewUserControllerImpl(appService *services.AppService) UserController {
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
// @Router /api/users/login/ [post]
func (c *UserControllerImpl) Login(ctx *gin.Context) {
	userLoginRequest := request.UserLoginRequest{}
	err := ctx.ShouldBindJSON(&userLoginRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &userLoginRequest),
		})
		return
	}
	user, _ := c.userService.FindByUsername(userLoginRequest.Username)
	utils.Logger.WithFields(logrus.Fields{
		"Username": user.Username,
		"UserId":   user.UserId,
		"ClientIP": ctx.ClientIP(),
	}).Info("登录")
	if !c.userService.VerifyPasswordByUsername(userLoginRequest.Username, userLoginRequest.Password) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "用户名或密码错误",
		})
		return
	}
	tokenString, err := c.userService.GetJwtTokenById(user)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"token": tokenString,
	})
}

// VerifyToken
// @Summary Token 鉴定
// @Description Token 鉴定
// @Tags 用户
// @Produce json
// @Param token path string true "token"
// @Router /api/users/token/{token} [get]
func (c *UserControllerImpl) VerifyToken(ctx *gin.Context) {
	id, err := c.userService.GetIdByJwtToken(ctx.Param("token"))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	}
	if id == "" {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "Token 无效",
		})
	} else {
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
// @Router /api/users/logout/ [post]
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
// @Router /api/users/register/ [post]
func (c *UserControllerImpl) Register(ctx *gin.Context) {
	createUserRequest := request.UserRegisterRequest{}
	err := ctx.ShouldBindJSON(&createUserRequest)
	if err != nil {
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
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "用户名或邮箱重复",
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Create
// @Summary 用户创建 *
// @Description 用户创建（管理员）
// @Tags 用户
// @Accept json
// @Produce json
// @Param input body request.UserCreateRequest true "UserCreateRequest"
// @Router /api/users/ [post]
func (c *UserControllerImpl) Create(ctx *gin.Context) {
	createUserRequest := request.UserCreateRequest{}
	err := ctx.ShouldBindJSON(&createUserRequest)
	if err != nil {
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
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "用户名或邮箱重复",
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Update
// @Summary 用户更新 *
// @Description 用户更新（管理员）
// @Tags 用户
// @Accept json
// @Produce json
// @Param input body request.UserUpdateRequest true "UserUpdateRequest"
// @Router /api/users/ [put]
func (c *UserControllerImpl) Update(ctx *gin.Context) {
	updateUserRequest := request.UserUpdateRequest{}
	err := ctx.ShouldBindJSON(&updateUserRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &updateUserRequest),
		})
		return
	}
	err = c.userService.Update(updateUserRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Delete
// @Summary 用户删除 *
// @Description 用户删除（管理员）
// @Tags 用户
// @Accept json
// @Produce json
// @Param input body request.UserDeleteRequest true "UserDeleteRequest"
// @Router /api/users/ [delete]
func (c *UserControllerImpl) Delete(ctx *gin.Context) {
	deleteUserRequest := request.UserDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteUserRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &deleteUserRequest),
		})
		return
	}
	_ = c.userService.Delete(deleteUserRequest.UserId)
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// FindById
// @Summary 用户查询（通过 Id）
// @Description 用户查询
// @Tags 用户
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /api/users/id/{id} [get]
func (c *UserControllerImpl) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	userResponse, _ := c.userService.FindById(id)
	if userResponse.UserId != "" {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"data": userResponse,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "用户不存在",
		})
	}
}

// FindByUsername
// @Summary 用户查询（通过 Username）
// @Description 用户查询
// @Tags 用户
// @Accept json
// @Produce json
// @Param username path string true "username"
// @Router /api/users/username/{username} [get]
func (c *UserControllerImpl) FindByUsername(ctx *gin.Context) {
	username := ctx.Param("username")
	userResponse, _ := c.userService.FindByUsername(username)
	if userResponse.UserId != "" {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"data": userResponse,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "用户不存在",
		})
	}
}

// FindAll
// @Summary 用户全部查询
// @Description 用户全部查询
// @Tags 用户
// @Accept json
// @Produce json
// @Router /api/users/ [get]
func (c *UserControllerImpl) FindAll(ctx *gin.Context) {
	userResponse, _ := c.userService.FindAll()
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": userResponse,
	})
}
