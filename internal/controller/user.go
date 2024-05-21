package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/model"
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
	Logout(ctx *gin.Context)
	Register(ctx *gin.Context)
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
	SaveAvatar(ctx *gin.Context)
	DeleteAvatar(ctx *gin.Context)
}

type UserController struct {
	userService  service.IUserService
	mediaService service.IMediaService
}

func NewUserController(appService *service.Service) IUserController {
	return &UserController{
		userService:  appService.UserService,
		mediaService: appService.MediaService,
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
	if err := ctx.ShouldBindJSON(&userLoginRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &userLoginRequest),
		})
		return
	}
	var user model.User
	if users, _, err := c.userService.Find(request.UserFindRequest{
		Username: userLoginRequest.Username,
	}); err == nil && len(users) > 0 {
		user = users[0]
	}
	if !c.userService.VerifyPasswordByUsername(userLoginRequest.Username, userLoginRequest.Password) {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "用户名或密码错误",
		})
		zap.L().Warn(fmt.Sprintf("User %s login failed", user.Username), zap.Uint("user_id", user.ID))
		return
	}
	tokenString, err := c.userService.GetJwtTokenByID(user)
	if err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{
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
	zap.L().Info(fmt.Sprintf("User %s login successful", user.Username), zap.Uint("user_id", user.ID))
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
	id, err := c.userService.GetIDByJwtToken(ctx.GetHeader("Authorization"))
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
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
	if err := ctx.ShouldBindJSON(&registerUserRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &registerUserRequest),
		})
		return
	}
	registerUserRequest.RemoteIP = ctx.RemoteIP()
	if err := c.userService.Register(registerUserRequest); err != nil {
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
	if err := ctx.ShouldBindJSON(&createUserRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &createUserRequest),
		})
		return
	}
	if err := c.userService.Create(createUserRequest); err != nil {
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
	if err := ctx.ShouldBindJSON(&updateUserRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &updateUserRequest),
		})
		return
	}
	updateUserRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	if err := c.userService.Update(updateUserRequest); err != nil {
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
	if err := ctx.ShouldBindQuery(&userFindRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &userFindRequest),
		})
		return
	}
	users, total, _ := c.userService.Find(userFindRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"data":  users,
		"total": total,
	})
}

// SaveAvatar
// @Summary 保存头像
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param file formData file true "avatar"
// @Router /users/{id}/avatar [post]
func (c *UserController) SaveAvatar(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	fileHeader, err := ctx.FormFile("file")
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	err = c.mediaService.SaveUserAvatar(id, fileHeader)
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

// DeleteAvatar
// @Summary 删除头像
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /users/{id}/avatar [delete]
func (c *UserController) DeleteAvatar(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	err := c.mediaService.DeleteUserAvatar(id)
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
