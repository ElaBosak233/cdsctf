package controller

import (
	"github.com/elabosak233/cloudsdale/internal/hub"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IGameController interface {
	BroadCast(ctx *gin.Context)
	Create(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Update(ctx *gin.Context)
	Find(ctx *gin.Context)
	GetChallengesByGameId(ctx *gin.Context)
	GetScoreboardByGameId(ctx *gin.Context)
}

type GameController struct {
	GameService service.IGameService
}

func NewGameController(appService *service.Service) IGameController {
	return &GameController{
		GameService: appService.GameService,
	}
}

// BroadCast
// @Summary 广播消息
// @Description	广播消息
// @Tags Game
// @Router /games/{id}/broadcast [get]
func (g *GameController) BroadCast(ctx *gin.Context) {
	id := convertor.ToInt64D(ctx.Param("id"), 0)
	if id != 0 {
		hub.ServeGameHub(ctx.Writer, ctx.Request, id)
	}
}

// ScoreBoard
// @Summary 计分板
// @Description	计分板
// @Tags Game
// @Router /games/{id}/scoreboard [get]
func (g *GameController) ScoreBoard(ctx *gin.Context) {
	//TODO implement me
	panic("implement me")
}

// Create
// @Summary 创建比赛
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 创建请求 body request.GameCreateRequest true "GameCreateRequest"
// @Router /games/ [post]
func (g *GameController) Create(ctx *gin.Context) {
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

// Delete
// @Summary 删除比赛
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 删除请求 body request.GameDeleteRequest true "GameDeleteRequest"
// @Router /games/{id} [delete]
func (g *GameController) Delete(ctx *gin.Context) {
	gameDeleteRequest := request.GameDeleteRequest{}
	gameDeleteRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := g.GameService.Delete(gameDeleteRequest)
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

// Update
// @Summary 更新比赛
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 更新请求 body request.GameUpdateRequest true "GameUpdateRequest"
// @Router /games/{id} [put]
func (g *GameController) Update(ctx *gin.Context) {
	gameUpdateRequest := request.GameUpdateRequest{}
	err := ctx.ShouldBindJSON(&gameUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameUpdateRequest),
		})
		return
	}
	gameUpdateRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
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
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 查找请求 query request.GameFindRequest false "GameFindRequest"
// @Router /games/ [get]
func (g *GameController) Find(ctx *gin.Context) {
	isEnabled := func() int {
		if ctx.GetInt64("UserLevel") < 3 && ctx.Query("is_enabled") == "-1" {
			return -1
		}
		return 1
	} // -1 代表忽略此条件，0 代表没被启用，1 代表被启用，默认状态下只查询被启用的比赛
	games, pageCount, total, err := g.GameService.Find(request.GameFindRequest{
		ID:        convertor.ToUintD(ctx.Query("id"), 0),
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

func (g *GameController) GetChallengesByGameId(ctx *gin.Context) {
	//TODO implement me
	panic("implement me")
}

func (g *GameController) GetScoreboardByGameId(ctx *gin.Context) {
	//TODO implement me
	panic("implement me")
}
