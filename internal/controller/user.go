package controller

import (
	"fmt"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/service"
	"github.com/elabosak233/pgshub/pkg/convertor"
	"github.com/elabosak233/pgshub/pkg/validator"
	"github.com/gin-gonic/gin"
	"github.com/mitchellh/mapstructure"
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
	UserService service.IUserService
}

func NewUserController(appService *service.Service) IUserController {
	return &UserController{
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
	user, _ := c.UserService.FindByUsername(userLoginRequest.Username)
	if !c.UserService.VerifyPasswordByUsername(userLoginRequest.Username, userLoginRequest.Password) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "用户名或密码错误",
		})
		zap.L().Warn(fmt.Sprintf("用户 %s 登录失败", user.Username), zap.Uint("user_id", user.ID))
		return
	}
	tokenString, err := c.UserService.GetJwtTokenById(user)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"token": tokenString,
	})
	zap.L().Info(fmt.Sprintf("用户 %s 登录成功", user.Username), zap.Uint("user_id", user.ID))
}

// VerifyToken
// @Summary Token 鉴定
// @Description
// @Tags 用户
// @Produce json
// @Param token path string true "token"
// @Router /api/users/token/{token} [get]
func (c *UserController) VerifyToken(ctx *gin.Context) {
	id, err := c.UserService.GetIdByJwtToken(ctx.Param("token"))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	}
	if id == 0 {
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
func (c *UserController) Logout(ctx *gin.Context) {
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
// @Param Authorization header string true "Authorization"
// @Param 创建请求 body request.UserCreateRequest true "UserCreateRequest"
// @Router /api/users/ [post]
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

// Update
// @Summary 用户更新（Role≤1 或 (Request)ID=(Authorization)ID）
// @Description 若 Role>1，则自动忽略 UserUpdateRequest 中的 Role 属性
// @Tags 用户
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param 更新请求 body request.UserUpdateRequest true "UserUpdateRequest"
// @Router /api/users/ [put]
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
	if ctx.GetInt64("UserLevel") <= 1 || ctx.GetUint("UserID") == updateUserRequest.ID {
		if ctx.GetInt64("UserLevel") > 1 {
			updateUserRequest.GroupID = ctx.GetUint("UserGroupID")
			updateUserRequest.Username = ""
		}
		err = c.UserService.Update(updateUserRequest)
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
		return
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusForbidden,
			"msg":  "权限不足",
		})
		return
	}
}

// Delete
// @Summary 用户删除（Role≤1 或 (Request)ID=(Authorization)ID）
// @Description
// @Tags 用户
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param input body request.UserDeleteRequest true "UserDeleteRequest"
// @Router /api/users/ [delete]
func (c *UserController) Delete(ctx *gin.Context) {
	deleteUserRequest := request.UserDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteUserRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &deleteUserRequest),
		})
		return
	}
	if ctx.GetInt64("UserLevel") <= 1 || ctx.GetUint("UserID") == deleteUserRequest.ID {
		_ = c.UserService.Delete(deleteUserRequest.ID)
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
func (c *UserController) Find(ctx *gin.Context) {
	userResponse, pageCount, total, _ := c.UserService.Find(request.UserFindRequest{
		ID:     convertor.ToUintD(ctx.Query("id"), 0),
		Email:  ctx.Query("email"),
		Name:   ctx.Query("name"),
		SortBy: ctx.QueryArray("sort_by"),
		Page:   convertor.ToIntD(ctx.Query("page"), 0),
		Size:   convertor.ToIntD(ctx.Query("size"), 0),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"data":  userResponse,
		"pages": pageCount,
		"total": total,
	})
}
