package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IChallengeController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
}

type ChallengeController struct {
	ChallengeService service.IChallengeService
}

func NewChallengeController(appService *service.Service) IChallengeController {
	return &ChallengeController{
		ChallengeService: appService.ChallengeService,
	}
}

// Find
// @Summary 题目查询
// @Description	只有当 Role≤2 并且 IsDetailed=1 时，才会提供题目的关键信息
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param input	query request.ChallengeFindRequest false "ChallengeFindRequest"
// @Router /challenges/ [get]
func (c *ChallengeController) Find(ctx *gin.Context) {
	isDetailed := func() *bool {
		if ctx.GetInt64("UserLevel") <= 2 {
			return convertor.ToBoolP(ctx.Query("is_detailed"))
		}
		return convertor.FalseP()
	}
	isPracticable := func() *bool {
		if ctx.GetInt64("UserLevel") <= 2 {
			return convertor.ToBoolP(ctx.Query("is_practicable"))
		}
		return convertor.TrueP()
	}
	challengeData, pageCount, total, _ := c.ChallengeService.Find(request.ChallengeFindRequest{
		Title:         ctx.Query("title"),
		CategoryID:    convertor.ToUintP(ctx.Query("category_id")),
		IsPracticable: isPracticable(),
		IDs:           convertor.ToUintSliceD(ctx.QueryArray("id"), make([]uint, 0)),
		IsDynamic:     convertor.ToBoolP(ctx.Query("is_dynamic")),
		Difficulty:    convertor.ToInt64D(ctx.Query("difficulty"), 0),
		UserID:        ctx.GetUint("UserID"),
		GameID:        convertor.ToUintP(ctx.Query("game_id")),
		TeamID:        convertor.ToUintP(ctx.Query("team_id")),
		IsDetailed:    isDetailed(),
		SubmissionQty: convertor.ToIntD(ctx.Query("submission_qty"), 0),
		Page:          convertor.ToIntD(ctx.Query("page"), 0),
		Size:          convertor.ToIntD(ctx.Query("size"), 0),
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
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 创建请求 body request.ChallengeCreateRequest true "ChallengeCreateRequest"
// @Router /challenges/ [post]
func (c *ChallengeController) Create(ctx *gin.Context) {
	createChallengeRequest := request.ChallengeCreateRequest{}
	err := ctx.ShouldBindJSON(&createChallengeRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &createChallengeRequest),
		})
		return
	}
	_ = c.ChallengeService.Create(createChallengeRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Update
// @Summary 更新题目（Role≤2）
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param request body request.ChallengeUpdateRequest true "ChallengeUpdateRequest"
// @Router /challenges/ [put]
func (c *ChallengeController) Update(ctx *gin.Context) {
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
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param request body request.ChallengeDeleteRequest true "ChallengeDeleteRequest"
// @Router /challenges/ [delete]
func (c *ChallengeController) Delete(ctx *gin.Context) {
	deleteChallengeRequest := request.ChallengeDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteChallengeRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &deleteChallengeRequest),
		})
		return
	}
	err = c.ChallengeService.Delete(deleteChallengeRequest.ID)
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
