package controllers

import (
	"github.com/elabosak233/pgshub/hubs"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/services"
	"github.com/elabosak233/pgshub/utils/convertor"
	"github.com/elabosak233/pgshub/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type GameController interface {
	BroadCast(ctx *gin.Context)
	Create(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Update(ctx *gin.Context)
	Find(ctx *gin.Context)
	GetChallengesByGameId(ctx *gin.Context)
	GetScoreboardByGameId(ctx *gin.Context)
}

type GameControllerImpl struct {
	GameService services.GameService
}

func NewGameControllerImpl(appService *services.Services) GameController {
	return &GameControllerImpl{
		GameService: appService.GameService,
	}
}

// BroadCast
// @Summary 广播消息
// @Description 广播消息
// @Tags 比赛
// @Router /api/games/:id/broadcast [get]
func (g *GameControllerImpl) BroadCast(ctx *gin.Context) {
	id := convertor.ToInt64D(ctx.Param("id"), 0)
	if id != 0 {
		hubs.ServeGameHub(ctx.Writer, ctx.Request, id)
	}
}

// Create
// @Summary 创建比赛（Role≤3）
// @Description
// @Tags 比赛
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param 创建请求 body request.GameCreateRequest true "GameCreateRequest"
// @Router /api/games/ [post]
func (g *GameControllerImpl) Create(ctx *gin.Context) {
	gameCreateRequest := request.GameCreateRequest{}
	err := ctx.ShouldBindJSON(&gameCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameCreateRequest),
		})
		return
	}
	err = g.GameService.Create(gameCreateRequest)
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

func (g *GameControllerImpl) Delete(ctx *gin.Context) {
	//TODO implement me
	panic("implement me")
}

func (g *GameControllerImpl) Update(ctx *gin.Context) {
	gameUpdateRequest := request.GameUpdateRequest{}
	err := ctx.ShouldBindJSON(&gameUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameUpdateRequest),
		})
		return
	}
	err = g.GameService.Update(gameUpdateRequest)
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

// Find
// @Summary 比赛查询
// @Description
// @Tags 比赛
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param 查找请求 query request.GameFindRequest false "GameFindRequest"
// @Router /api/games/ [get]
func (g *GameControllerImpl) Find(ctx *gin.Context) {
	isEnabled := func() int {
		if ctx.GetInt64("UserRole") < 3 && ctx.Query("is_enabled") == "-1" {
			return -1
		}
		return 1
	} // -1 代表忽略此条件，0 代表没被启用，1 代表被启用，默认状态下只查询被启用的比赛
	games, pageCount, total, err := g.GameService.Find(request.GameFindRequest{
		ID:        int64(convertor.ToIntD(ctx.Query("id"), 0)),
		Title:     ctx.Query("title"),
		IsEnabled: isEnabled(),
		Size:      convertor.ToIntD(ctx.Query("size"), 0),
		Page:      convertor.ToIntD(ctx.Query("page"), 0),
		SortBy:    ctx.QueryArray("sort_by"),
	})
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"data":  games,
		"total": total,
		"pages": pageCount,
	})
}

func (g *GameControllerImpl) GetChallengesByGameId(ctx *gin.Context) {
	//TODO implement me
	panic("implement me")
}

func (g *GameControllerImpl) GetScoreboardByGameId(ctx *gin.Context) {
	//TODO implement me
	panic("implement me")
}
