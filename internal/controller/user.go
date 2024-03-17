package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"net/http"
)

type IUserController interface {
	Login(ctx *gin.Context)
	VerifyToken(ctx *gin.Context)
	Logout(ctx *gin.Context)
	Register(ctx *gin.Context)
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
}

type UserController struct {
	userService service.IUserService
}

func NewUserController(appService *service.Service) IUserController {
	return &UserController{
		userService: appService.UserService,
	}
}

// Login
// @Summary	用户登录
// @Description
// @Tags User
// @Accept json
// @Produce	json
// @Param 登录请求 body request.UserLoginRequest true "UserLoginRequest"
// @Router /users/login [post]
func (c *UserController) Login(ctx *gin.Context) {
	userLoginRequest := request.UserLoginRequest{}
	err := ctx.ShouldBindJSON(&userLoginRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &userLoginRequest),
		})
		return
	}
	user, _ := c.userService.FindByUsername(userLoginRequest.Username)
	if !c.userService.VerifyPasswordByUsername(userLoginRequest.Username, userLoginRequest.Password) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "用户名或密码错误",
		})
		zap.L().Warn(fmt.Sprintf("用户 %s 登录失败", user.Username), zap.Uint("user_id", user.ID))
		return
	}
	tokenString, err := c.userService.GetJwtTokenById(user)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"data":  user,
		"token": tokenString,
	})
	zap.L().Info(fmt.Sprintf("用户 %s 登录成功", user.Username), zap.Uint("user_id", user.ID))
}

// VerifyToken
// @Summary	Token 鉴定
// @Description
// @Tags User
// @Produce	json
// @Param token	path string	true "token"
// @Router /users/token/{token} [get]
func (c *UserController) VerifyToken(ctx *gin.Context) {
	id, err := c.userService.GetIdByJwtToken(ctx.Param("token"))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	if id == 0 {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "Token 无效",
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"id":   id,
	})
}

// Logout
// @Summary	用户登出
// @Description
// @Tags User
// @Accept json
// @Produce	json
// @Security ApiKeyAuth
// @Router /users/logout [post]
func (c *UserController) Logout(ctx *gin.Context) {
	id, err := c.userService.GetIdByJwtToken(ctx.GetHeader("Authorization"))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"id":   id,
	})
}

// Register
// @Summary	用户注册
// @Description
// @Tags User
// @Accept json
// @Produce	json
// @Param input	body request.UserRegisterRequest true "UserRegisterRequest"
// @Router /users/register [post]
func (c *UserController) Register(ctx *gin.Context) {
	registerUserRequest := request.UserRegisterRequest{}
	err := ctx.ShouldBindJSON(&registerUserRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &registerUserRequest),
		})
		return
	}
	registerUserRequest.RemoteIP = ctx.RemoteIP()
	err = c.userService.Register(registerUserRequest)
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
// @Summary	用户创建
// @Description
// @Tags User
// @Accept json
// @Produce	json
// @Security ApiKeyAuth
// @Param 创建请求 body request.UserCreateRequest true "UserCreateRequest"
// @Router /users/ [post]
func (c *UserController) Create(ctx *gin.Context) {
	createUserRequest := request.UserCreateRequest{}
	err := ctx.ShouldBindJSON(&createUserRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &createUserRequest),
		})
		return
	}
	err = c.userService.Create(createUserRequest)
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

// Update
// @Summary	用户更新
// @Description
// @Tags User
// @Accept json
// @Produce	json
// @Security ApiKeyAuth
// @Param 更新请求 body request.UserUpdateRequest true "UserUpdateRequest"
// @Router /users/{id} [put]
func (c *UserController) Update(ctx *gin.Context) {
	updateUserRequest := request.UserUpdateRequest{}
	err := ctx.ShouldBindJSON(&updateUserRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &updateUserRequest),
		})
		return
	}
	updateUserRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.userService.Update(updateUserRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
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
// @Summary	用户删除
// @Description
// @Tags User
// @Accept json
// @Produce	json
// @Security ApiKeyAuth
// @Param input	body request.UserDeleteRequest true "UserDeleteRequest"
// @Router /users/{id} [delete]
func (c *UserController) Delete(ctx *gin.Context) {
	deleteUserRequest := request.UserDeleteRequest{}
	deleteUserRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	_ = c.userService.Delete(deleteUserRequest.ID)
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Find
// @Summary	用户查询
// @Description
// @Tags User
// @Accept json
// @Produce	json
// @Param input	query request.UserFindRequest false	"UserFindRequest"
// @Router /users/ [get]
func (c *UserController) Find(ctx *gin.Context) {
	userFindRequest := request.UserFindRequest{}
	err := ctx.ShouldBindQuery(&userFindRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &userFindRequest),
		})
		return
	}
	userResponse, pageCount, total, _ := c.userService.Find(userFindRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"data":  userResponse,
		"pages": pageCount,
		"total": total,
	})
}
