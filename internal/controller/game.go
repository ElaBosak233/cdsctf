package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/cache"
	"github.com/elabosak233/cloudsdale/internal/extension/broadcast"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
	"time"
)

type IGameController interface {
	Create(ctx *gin.Context)
	Find(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Update(ctx *gin.Context)
	BroadCast(ctx *gin.Context)
	FindTeam(ctx *gin.Context)
	FindTeamByID(ctx *gin.Context)
	CreateTeam(ctx *gin.Context)
	UpdateTeam(ctx *gin.Context)
	DeleteTeam(ctx *gin.Context)
	FindChallenge(ctx *gin.Context)
	CreateChallenge(ctx *gin.Context)
	UpdateChallenge(ctx *gin.Context)
	DeleteChallenge(ctx *gin.Context)
	FindNotice(ctx *gin.Context)
	CreateNotice(ctx *gin.Context)
	UpdateNotice(ctx *gin.Context)
	DeleteNotice(ctx *gin.Context)
	SavePoster(ctx *gin.Context)
	DeletePoster(ctx *gin.Context)
}

type GameController struct {
	gameService          service.IGameService
	gameChallengeService service.IGameChallengeService
	gameTeamService      service.IGameTeamService
	challengeService     service.IChallengeService
	teamService          service.ITeamService
	noticeService        service.INoticeService
	mediaService         service.IMediaService
}

func NewGameController(appService *service.Service) IGameController {
	return &GameController{
		gameService:          appService.GameService,
		gameChallengeService: appService.GameChallengeService,
		gameTeamService:      appService.GameTeamService,
		challengeService:     appService.ChallengeService,
		teamService:          appService.TeamService,
		noticeService:        appService.NoticeService,
		mediaService:         appService.MediaService,
	}
}

// BroadCast
// @Summary 广播消息
// @Description	广播消息
// @Tags Game
// @Router /games/{id}/broadcast [get]
func (g *GameController) BroadCast(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	if id != 0 {
		broadcast.ServeGameHub(ctx.Writer, ctx.Request, id)
	}
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
	gameChallengeFindRequest := request.GameChallengeFindRequest{}
	_ = ctx.ShouldBindQuery(&gameChallengeFindRequest)
	gameChallengeFindRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	value, exist := cache.C().Get(fmt.Sprintf("game_challenges:%s", utils.HashStruct(gameChallengeFindRequest)))
	if !exist {
		challenges, err := g.gameChallengeService.Find(gameChallengeFindRequest)
		if err != nil {
			ctx.JSON(http.StatusBadRequest, gin.H{
				"code": http.StatusBadRequest,
			})
			return
		}
		value = gin.H{
			"code": http.StatusOK,
			"data": challenges,
		}
		cache.C().Set(
			fmt.Sprintf("game_challenges:%s", utils.HashStruct(gameChallengeFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
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
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameChallengeCreateRequest),
		})
		return
	}
	gameChallengeCreateRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	err = g.gameChallengeService.Create(gameChallengeCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_challenges")
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
	if err := ctx.ShouldBindJSON(&gameChallengeUpdateRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameChallengeUpdateRequest),
		})
		return
	}
	gameChallengeUpdateRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	gameChallengeUpdateRequest.ChallengeID = convertor.ToUintD(ctx.Param("challenge_id"), 0)
	err := g.gameChallengeService.Update(gameChallengeUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_challenges")
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
	err := g.gameChallengeService.Delete(gameChallengeDeleteRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_challenges")
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
	gameTeamFindRequest := request.GameTeamFindRequest{}
	gameTeamFindRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	value, exist := cache.C().Get(fmt.Sprintf("game_teams:%s", utils.HashStruct(gameTeamFindRequest)))
	if !exist {
		teams, total, err := g.gameTeamService.Find(gameTeamFindRequest)
		if err != nil {
			ctx.JSON(http.StatusBadRequest, gin.H{
				"code": http.StatusBadRequest,
			})
			return
		}
		value = gin.H{
			"code":  http.StatusOK,
			"data":  teams,
			"total": total,
		}
		cache.C().Set(
			fmt.Sprintf("game_teams:%s", utils.HashStruct(gameTeamFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
}

// FindTeamByID
// @Summary 查询比赛的团队
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/teams/{team_id} [get]
func (g *GameController) FindTeamByID(ctx *gin.Context) {
	gameTeamFindRequest := request.GameTeamFindRequest{
		GameID: convertor.ToUintD(ctx.Param("id"), 0),
		TeamID: convertor.ToUintD(ctx.Param("team_id"), 0),
	}
	value, exist := cache.C().Get(fmt.Sprintf("game_teams:%s", utils.HashStruct(gameTeamFindRequest)))
	if !exist {
		team, err := g.gameTeamService.FindByID(gameTeamFindRequest)
		if err != nil {
			ctx.JSON(http.StatusBadRequest, gin.H{
				"code": http.StatusBadRequest,
			})
			return
		}
		value = gin.H{
			"code": http.StatusOK,
			"data": team,
		}
		cache.C().Set(
			fmt.Sprintf("game_teams:%s", utils.HashStruct(gameTeamFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
}

// CreateTeam
// @Summary 加入比赛
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 加入请求 body request.GameTeamCreateRequest true "GameTeamCreateRequest"
// @Router /games/{id}/teams [post]
func (g *GameController) CreateTeam(ctx *gin.Context) {
	gameTeamCreateRequest := request.GameTeamCreateRequest{}
	if err := ctx.ShouldBindJSON(&gameTeamCreateRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameTeamCreateRequest),
		})
		return
	}
	user := ctx.MustGet("user").(*model.User)
	gameTeamCreateRequest.UserID = user.ID
	gameTeamCreateRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := g.gameTeamService.Create(gameTeamCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_teams")
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
// @Param 允许加入请求 body request.GameTeamUpdateRequest true "GameTeamUpdateRequest"
// @Router /games/{id}/teams/{team_id} [put]
func (g *GameController) UpdateTeam(ctx *gin.Context) {
	gameTeamUpdateRequest := request.GameTeamUpdateRequest{}
	gameTeamUpdateRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	gameTeamUpdateRequest.TeamID = convertor.ToUintD(ctx.Param("team_id"), 0)
	err := ctx.ShouldBindJSON(&gameTeamUpdateRequest)
	err = g.gameTeamService.Update(gameTeamUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_teams")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteTeam
// @Summary 删除比赛的团队
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/teams/{team_id} [delete]
func (g *GameController) DeleteTeam(ctx *gin.Context) {
	gameTeamDeleteRequest := request.GameTeamDeleteRequest{}
	gameTeamDeleteRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	gameTeamDeleteRequest.TeamID = convertor.ToUintD(ctx.Param("team_id"), 0)
	err := g.gameTeamService.Delete(gameTeamDeleteRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_teams")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// FindNotice
// @Summary 查询比赛的通知
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/notices [get]
func (g *GameController) FindNotice(ctx *gin.Context) {
	noticeFindRequest := request.NoticeFindRequest{}
	noticeFindRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	_ = ctx.ShouldBindQuery(&noticeFindRequest)
	value, exist := cache.C().Get(fmt.Sprintf("game_notices:%s", utils.HashStruct(noticeFindRequest)))
	if !exist {
		notices, total, err := g.noticeService.Find(noticeFindRequest)
		if err != nil {
			ctx.JSON(http.StatusBadRequest, gin.H{
				"code": http.StatusBadRequest,
			})
			return
		}
		value = gin.H{
			"code":  http.StatusOK,
			"data":  notices,
			"total": total,
		}
		cache.C().Set(
			fmt.Sprintf("game_notices:%s", utils.HashStruct(noticeFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
}

// CreateNotice
// @Summary 添加比赛的通知
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/notices [post]
func (g *GameController) CreateNotice(ctx *gin.Context) {
	noticeCreateRequest := request.NoticeCreateRequest{}
	err := ctx.ShouldBindJSON(&noticeCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &noticeCreateRequest),
		})
		return
	}
	noticeCreateRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	err = g.noticeService.Create(noticeCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_notices")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// UpdateNotice
// @Summary 更新比赛的通知
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/notices/{notice_id} [put]
func (g *GameController) UpdateNotice(ctx *gin.Context) {
	noticeUpdateRequest := request.NoticeUpdateRequest{}
	noticeUpdateRequest.GameID = convertor.ToUintD(ctx.Param("id"), 0)
	noticeUpdateRequest.ID = convertor.ToUintD(ctx.Param("notice_id"), 0)
	err := ctx.ShouldBindJSON(&noticeUpdateRequest)
	err = g.noticeService.Update(noticeUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_notices")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteNotice
// @Summary 删除比赛的通知
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/notices/{notice_id} [delete]
func (g *GameController) DeleteNotice(ctx *gin.Context) {
	noticeDeleteRequest := request.NoticeDeleteRequest{}
	noticeDeleteRequest.ID = convertor.ToUintD(ctx.Param("notice_id"), 0)
	err := g.noticeService.Delete(noticeDeleteRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("game_notices")
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
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameCreateRequest),
		})
		return
	}
	err = g.gameService.Create(gameCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("games")
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
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("games")
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
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameUpdateRequest),
		})
		return
	}
	gameUpdateRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err = g.gameService.Update(gameUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("games")
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
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &gameFindRequest),
		})
		return
	}
	if ok && isEnabled.(bool) {
		gameFindRequest.IsEnabled = convertor.TrueP()
	}
	value, exist := cache.C().Get(fmt.Sprintf("games:%s", utils.HashStruct(gameFindRequest)))
	if !exist {
		games, total, err := g.gameService.Find(gameFindRequest)
		if err != nil {
			ctx.JSON(http.StatusBadRequest, gin.H{
				"code": http.StatusBadRequest,
			})
			return
		}
		value = gin.H{
			"code":  http.StatusOK,
			"data":  games,
			"total": total,
		}
		cache.C().Set(
			fmt.Sprintf("games:%s", utils.HashStruct(gameFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
}

// SavePoster
// @Summary 保存头图
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param file formData file true "poster"
// @Router /games/{id}/poster [post]
func (g *GameController) SavePoster(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	fileHeader, err := ctx.FormFile("file")
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	err = g.mediaService.SaveGamePoster(id, fileHeader)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("games")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeletePoster
// @Summary 删除海报
// @Description
// @Tags Game
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /games/{id}/poster [delete]
func (g *GameController) DeletePoster(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	err := g.mediaService.DeleteGamePoster(id)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("games")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
