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
	challengeService service.IChallengeService
}

func NewChallengeController(appService *service.Service) IChallengeController {
	return &ChallengeController{
		challengeService: appService.ChallengeService,
	}
}

// Find
// @Summary 题目查询
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param input	query request.ChallengeFindRequest false "ChallengeFindRequest"
// @Router /challenges/ [get]
func (c *ChallengeController) Find(ctx *gin.Context) {
	isDetailed := ctx.GetBool("is_detailed")
	isPracticable := ctx.GetBool("is_practicable")
	challenges, pageCount, total, _ := c.challengeService.Find(request.ChallengeFindRequest{
		Title:         ctx.Query("title"),
		CategoryID:    convertor.ToUintP(ctx.Query("category_id")),
		IsPracticable: &isPracticable,
		IDs:           convertor.ToUintSliceD(ctx.QueryArray("id"), make([]uint, 0)),
		IsDynamic:     convertor.ToBoolP(ctx.Query("is_dynamic")),
		Difficulty:    convertor.ToInt64D(ctx.Query("difficulty"), 0),
		UserID:        ctx.GetUint("UserID"),
		GameID:        convertor.ToUintP(ctx.Query("game_id")),
		TeamID:        convertor.ToUintP(ctx.Query("team_id")),
		IsDetailed:    &isDetailed,
		SubmissionQty: convertor.ToIntD(ctx.Query("submission_qty"), 0),
		Page:          convertor.ToIntD(ctx.Query("page"), 0),
		Size:          convertor.ToIntD(ctx.Query("size"), 0),
		SortBy:        ctx.QueryArray("sort_by"),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"total": total,
		"data":  challenges,
	})
}

// Create
// @Summary 创建题目
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
	_ = c.challengeService.Create(createChallengeRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Update
// @Summary 更新题目
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param request body request.ChallengeUpdateRequest true "ChallengeUpdateRequest"
// @Router /challenges/{id} [put]
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
	updateChallengeRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.challengeService.Update(updateChallengeRequest)
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
// @Summary 删除题目
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param request body request.ChallengeDeleteRequest true "ChallengeDeleteRequest"
// @Router /challenges/{id} [delete]
func (c *ChallengeController) Delete(ctx *gin.Context) {
	deleteChallengeRequest := request.ChallengeDeleteRequest{}
	deleteChallengeRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.challengeService.Delete(deleteChallengeRequest.ID)
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
