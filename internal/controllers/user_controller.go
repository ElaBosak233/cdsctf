package controllers

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"github.com/mitchellh/mapstructure"
	"github.com/sirupsen/logrus"
	"net/http"
)

type UserController interface {
	Login(ctx *gin.Context)
	VerifyToken(ctx *gin.Context)
	Logout(ctx *gin.Context)
	Register(ctx *gin.Context)
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
}

type UserControllerImpl struct {
	UserService services.UserService
}

func NewUserControllerImpl(appService *services.AppService) UserController {
	return &UserControllerImpl{
		UserService: appService.UserService,
	}
}

// Login
// @Summary 用户登录
// @Description
// @Tags 用户
// @Accept json
// @Produce json
// @Param 登录请求 body request.UserLoginRequest true "UserLoginRequest"
// @Router /api/users/login [post]
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
	user, _ := c.UserService.FindByUsername(userLoginRequest.Username)
	utils.Logger.WithFields(logrus.Fields{
		"Username": user.Username,
		"UserId":   user.UserId,
		"ClientIP": ctx.ClientIP(),
	}).Info("登录")
	if !c.UserService.VerifyPasswordByUsername(userLoginRequest.Username, userLoginRequest.Password) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "用户名或密码错误",
		})
		return
	}
	tokenString, err := c.UserService.GetJwtTokenById(user)
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
// @Description
// @Tags 用户
// @Produce json
// @Param token path string true "token"
// @Router /api/users/token/{token} [get]
func (c *UserControllerImpl) VerifyToken(ctx *gin.Context) {
	id, err := c.UserService.GetIdByJwtToken(ctx.Param("token"))
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
// @Description
// @Tags 用户
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Router /api/users/logout [post]
func (c *UserControllerImpl) Logout(ctx *gin.Context) {
	id, err := c.UserService.GetIdByJwtToken(ctx.GetHeader("Authorization"))
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
// @Summary 用户注册
// @Description
// @Tags 用户
// @Accept json
// @Produce json
// @Param input body request.UserRegisterRequest true "UserRegisterRequest"
// @Router /api/users/register [post]
func (c *UserControllerImpl) Register(ctx *gin.Context) {
	registerUserRequest := request.UserRegisterRequest{}
	err := ctx.ShouldBindJSON(&registerUserRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &registerUserRequest),
		})
		return
	}
	createUserRequest := request.UserCreateRequest{}
	_ = mapstructure.Decode(registerUserRequest, &createUserRequest)
	err = c.UserService.Create(createUserRequest)
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
// @Summary 用户创建（Role<=1）
// @Description
// @Tags 用户
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param 创建请求 body request.UserCreateRequest true "UserCreateRequest"
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
	err = c.UserService.Create(createUserRequest)
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
// @Summary 用户更新（Role≤1 或 (Request)UserId=(PgsToken)UserId）
// @Description 若 Role>1，则自动忽略 UserUpdateRequest 中的 Role 属性
// @Tags 用户
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param 更新请求 body request.UserUpdateRequest true "UserUpdateRequest"
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
	if ctx.GetInt("UserRole") <= 1 || ctx.GetString("UserId") == updateUserRequest.UserId {
		if ctx.GetInt("UserRole") > 1 {
			updateUserRequest.Role = ctx.GetInt("UserRole")
		}
		err = c.UserService.Update(updateUserRequest)
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
		return
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusForbidden,
			"msg":  "权限不足",
		})
	}
}

// Delete
// @Summary 用户删除（Role≤1 或 (Request)UserId=(PgsToken)UserId）
// @Description
// @Tags 用户
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
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
	if ctx.GetInt("UserRole") <= 1 || ctx.GetString("UserId") == deleteUserRequest.UserId {
		_ = c.UserService.Delete(deleteUserRequest.UserId)
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusForbidden,
			"msg":  "权限不足",
		})
	}
}

// Find
// @Summary 用户查询
// @Description
// @Tags 用户
// @Accept json
// @Produce json
// @Param input query request.UserFindRequest false "UserFindRequest"
// @Router /api/users/ [get]
func (c *UserControllerImpl) Find(ctx *gin.Context) {
	if ctx.Query("id") == "" && ctx.Query("username") == "" && ctx.Query("email") == "" {
		userResponse, pageCount, _ := c.UserService.Find(request.UserFindRequest{
			Role: utils.ParseIntParam(ctx.Query("UserRole"), -1),
			Name: ctx.Query("name"),
			Page: utils.ParseIntParam(ctx.Query("page"), -1),
			Size: utils.ParseIntParam(ctx.Query("size"), -1),
		})
		ctx.JSON(http.StatusOK, gin.H{
			"code":  http.StatusOK,
			"data":  userResponse,
			"pages": pageCount,
		})
	} else if ctx.Query("id") != "" && ctx.Query("username") == "" && ctx.Query("email") == "" {
		userResponse, _ := c.UserService.FindById(ctx.Query("id"))
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"data": userResponse,
		})
	} else if ctx.Query("id") == "" && ctx.Query("username") != "" && ctx.Query("email") == "" {
		userResponse, _ := c.UserService.FindByUsername(ctx.Query("username"))
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"data": userResponse,
		})
	} else if ctx.Query("id") == "" && ctx.Query("username") == "" && ctx.Query("email") != "" {
		userResponse, _ := c.UserService.FindByEmail(ctx.Query("email"))
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"data": userResponse,
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "参数错误",
		})
	}
}
