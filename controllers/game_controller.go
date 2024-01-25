package controllers

import (
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/services"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type GameController interface {
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

func NewGameControllerImpl(appService *services.AppService) GameController {
	return &GameControllerImpl{
		GameService: appService.GameService,
	}
}

// Create
// @Summary 创建比赛（Role≤3）
// @Description
// @Tags 比赛
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param 创建请求 body request.GameCreateRequest true "GameCreateRequest"
// @Router /api/games/ [post]
func (g *GameControllerImpl) Create(ctx *gin.Context) {
	gameCreateRequest := request.GameCreateRequest{}
	err := ctx.ShouldBindJSON(&gameCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &gameCreateRequest),
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
	//TODO implement me
	panic("implement me")
}

// Find
// @Summary 比赛查询
// @Description
// @Tags 比赛
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param 查找请求 query request.GameFindRequest false "GameFindRequest"
// @Router /api/games/ [get]
func (g *GameControllerImpl) Find(ctx *gin.Context) {
	games, pageCount, total, err := g.GameService.Find(request.GameFindRequest{
		GameId: int64(utils.ParseIntParam(ctx.Query("id"), 0)),
		Title:  ctx.Query("title"),
		Size:   utils.ParseIntParam(ctx.Query("size"), 0),
		Page:   utils.ParseIntParam(ctx.Query("page"), 0),
		SortBy: ctx.QueryArray("sort_by"),
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
