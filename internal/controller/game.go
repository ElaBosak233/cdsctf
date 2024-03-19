package controller

import (
	"github.com/elabosak233/cloudsdale/internal/hub"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IGameController interface {
	Create(ctx *gin.Context)
	Find(ctx *gin.Context)
	FindByID(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Update(ctx *gin.Context)
	BroadCast(ctx *gin.Context)
	Scoreboard(ctx *gin.Context)
	FindTeam(ctx *gin.Context)
	CreateTeam(ctx *gin.Context)
	UpdateTeam(ctx *gin.Context)
	FindChallenge(ctx *gin.Context)
	CreateChallenge(ctx *gin.Context)
	UpdateChallenge(ctx *gin.Context)
	DeleteChallenge(ctx *gin.Context)
}

type GameController struct {
	gameService      service.IGameService
	challengeService service.IChallengeService
	teamService      service.ITeamService
}

func NewGameController(appService *service.Service) IGameController {
	return &GameController{
		gameService:      appService.GameService,
		challengeService: appService.ChallengeService,
		teamService:      appService.TeamService,
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

// Scoreboard
// @Summary 查询比赛的积分榜
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/scoreboard [get]
func (g *GameController) Scoreboard(ctx *gin.Context) {
	scoreboard, err := g.gameService.Scoreboard(convertor.ToUintD(ctx.Param("id"), 0))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": scoreboard,
	})
}

// FindChallenge
// @Summary 查询比赛的挑战
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/challenges [get]
func (g *GameController) FindChallenge(ctx *gin.Context) {
	challenges, err := g.gameService.FindChallenge(request.GameChallengeFindRequest{
		GameID: convertor.ToUintD(ctx.Param("id"), 0),
		TeamID: convertor.ToUintD(ctx.Query("team_id"), 0),
	})
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": challenges,
	})
}

// CreateChallenge
// @Summary 添加比赛的挑战
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/challenges [post]
func (g *GameController) CreateChallenge(ctx *gin.Context) {
	gameChallengeCreateRequest := request.GameChallengeCreateRequest{}
	err := ctx.ShouldBindJSON(&gameChallengeCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameChallengeCreateRequest),
		})
		return
	}
	gameChallengeCreateRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	err = g.gameService.CreateChallenge(gameChallengeCreateRequest)
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

// UpdateChallenge
// @Summary 更新比赛的挑战
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/challenges/{challenge_id} [put]
func (g *GameController) UpdateChallenge(ctx *gin.Context) {
	gameChallengeUpdateRequest := request.GameChallengeUpdateRequest{}
	err := ctx.ShouldBindJSON(&gameChallengeUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameChallengeUpdateRequest),
		})
		return
	}
	gameChallengeUpdateRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	gameChallengeUpdateRequest.ChallengeID = convertor.ToUintD(ctx.Param("challenge_id"), 0)
	err = g.gameService.UpdateChallenge(gameChallengeUpdateRequest)
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

// DeleteChallenge
// @Summary 删除比赛的挑战
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/challenges/{challenge_id} [delete]
func (g *GameController) DeleteChallenge(ctx *gin.Context) {
	gameChallengeDeleteRequest := request.GameChallengeDeleteRequest{}
	gameChallengeDeleteRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	gameChallengeDeleteRequest.ChallengeID = convertor.ToUintD(ctx.Param("challenge_id"), 0)
	err := g.gameService.DeleteChallenge(gameChallengeDeleteRequest)
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

// FindTeam
// @Summary 查询比赛的团队
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/teams [get]
func (g *GameController) FindTeam(ctx *gin.Context) {
	teams, pageCount, total, err := g.teamService.Find(request.TeamFindRequest{
		GameID: convertor.ToUintP(ctx.Param("id")),
	})
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"data":  teams,
		"total": total,
		"pages": pageCount,
	})
}

// CreateTeam
// @Summary 加入比赛
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 加入请求 body request.GameJoinRequest true "GameJoinRequest"
// @Router /games/{id}/teams [post]
func (g *GameController) CreateTeam(ctx *gin.Context) {
	gameJoinRequest := request.GameJoinRequest{}
	err := ctx.ShouldBindJSON(&gameJoinRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameJoinRequest),
		})
		return
	}
	gameJoinRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	gameJoinRequest.TeamID = convertor.ToUintD(ctx.Param("team_id"), 0)
	err = g.gameService.Join(gameJoinRequest)
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

// UpdateTeam
// @Summary 允许加入比赛
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 允许加入请求 body request.GameAllowJoinRequest true "GameAllowJoinRequest"
// @Router /games/{id}/teams/{team_id} [put]
func (g *GameController) UpdateTeam(ctx *gin.Context) {
	gameAllowJoinRequest := request.GameAllowJoinRequest{}
	gameAllowJoinRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	gameAllowJoinRequest.TeamID = convertor.ToUintD(ctx.Param("team_id"), 0)
	err := g.gameService.AllowJoin(gameAllowJoinRequest)
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
	err = g.gameService.Create(gameCreateRequest)
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
	err := g.gameService.Delete(gameDeleteRequest)
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
	err = g.gameService.Update(gameUpdateRequest)
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
	isEnabled, ok := ctx.Get("is_enabled")
	gameFindRequest := request.GameFindRequest{}
	err := ctx.ShouldBindQuery(&gameFindRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameFindRequest),
		})
		return
	}
	if ok && isEnabled.(bool) {
		gameFindRequest.IsEnabled = convertor.TrueP()
	}
	games, pageCount, total, err := g.gameService.Find(gameFindRequest)
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

// FindByID
// @Summary 比赛查询
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 查找请求 query request.GameFindRequest false "GameFindRequest"
// @Router /games/{id} [get]
func (g *GameController) FindByID(ctx *gin.Context) {
	isEnabled, ok := ctx.Get("is_enabled")
	gameFindRequest := request.GameFindRequest{}
	if ok && isEnabled.(bool) {
		gameFindRequest.IsEnabled = convertor.TrueP()
	}
	gameFindRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	games, _, _, err := g.gameService.Find(gameFindRequest)
	if err != nil || len(games) == 0 {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": games[0],
	})
}
