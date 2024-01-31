package controllers

import (
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/services"
	"github.com/elabosak233/pgshub/utils/convertor"
	"github.com/elabosak233/pgshub/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ChallengeController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
}

type ChallengeControllerImpl struct {
	ChallengeService services.ChallengeService
}

func NewChallengeController(appService *services.Services) ChallengeController {
	return &ChallengeControllerImpl{
		ChallengeService: appService.ChallengeService,
	}
}

// Find
// @Summary 题目查询
// @Description 只有当 Role≤2 并且 IsDetailed=1 时，才会提供题目的关键信息
// @Tags 题目
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param input query request.ChallengeFindRequest false "ChallengeFindRequest"
// @Router /api/challenges/ [get]
func (c *ChallengeControllerImpl) Find(ctx *gin.Context) {
	isDetailed := func() int {
		if ctx.GetInt64("UserRole") <= 2 && convertor.ToIntD(ctx.Query("is_detailed"), 0) == 1 {
			return 1
		}
		return 0
	}
	isPracticable := func() int {
		if ctx.GetInt64("UserRole") <= 2 {
			switch convertor.ToIntD(ctx.Query("is_practicable"), -1) {
			case 0:
				return 0
			case 1:
				return 1
			case -1:
				return -1
			}
		}
		return 1
	}
	challengeData, pageCount, total, _ := c.ChallengeService.Find(request.ChallengeFindRequest{
		Title:         ctx.Query("title"),
		Category:      ctx.Query("category"),
		IsPracticable: isPracticable(),
		ChallengeIds:  convertor.ToInt64SliceD(ctx.QueryArray("id"), make([]int64, 0)),
		IsDynamic:     convertor.ToIntD(ctx.Query("is_dynamic"), -1),
		Difficulty:    convertor.ToInt64D(ctx.Query("difficulty"), -1),
		UserId:        ctx.GetInt64("UserId"),
		GameId:        convertor.ToInt64D(ctx.Query("game_id"), -1),
		TeamId:        convertor.ToInt64D(ctx.Query("team_id"), -1),
		IsDetailed:    isDetailed(),
		Page:          convertor.ToIntD(ctx.Query("page"), -1),
		Size:          convertor.ToIntD(ctx.Query("size"), -1),
		SortBy:        ctx.QueryArray("sort_by"),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"total": total,
		"data":  challengeData,
	})
}

// Create
// @Summary 创建题目（Role≤2）
// @Description
// @Tags 题目
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param 创建请求 body request.ChallengeCreateRequest true "ChallengeCreateRequest"
// @Router /api/challenges/ [post]
func (c *ChallengeControllerImpl) Create(ctx *gin.Context) {
	createChallengeRequest := request.ChallengeCreateRequest{}
	err := ctx.ShouldBindJSON(&createChallengeRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &createChallengeRequest),
		})
		return
	}
	_ = c.ChallengeService.Create(createChallengeRequest)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Update
// @Summary 更新题目（Role≤2）
// @Description
// @Tags 题目
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param request body request.ChallengeUpdateRequest true "ChallengeUpdateRequest"
// @Router /api/challenges/ [put]
func (c *ChallengeControllerImpl) Update(ctx *gin.Context) {
	var updateChallengeRequest request.ChallengeUpdateRequest
	err := ctx.ShouldBindJSON(&updateChallengeRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &updateChallengeRequest),
		})
		return
	}
	err = c.ChallengeService.Update(updateChallengeRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Delete
// @Summary 删除题目（Role≤2）
// @Description
// @Tags 题目
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param request body request.ChallengeDeleteRequest true "ChallengeDeleteRequest"
// @Router /api/challenges/ [delete]
func (c *ChallengeControllerImpl) Delete(ctx *gin.Context) {
	deleteChallengeRequest := request.ChallengeDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteChallengeRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &deleteChallengeRequest),
		})
		return
	}
	err = c.ChallengeService.Delete(deleteChallengeRequest.ChallengeId)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "删除失败",
		})
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
