package implements

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ChallengeControllerImpl struct {
	challengeService services.ChallengeService
}

func NewChallengeController(appService *services.AppService) controllers.ChallengeController {
	return &ChallengeControllerImpl{
		challengeService: appService.ChallengeService,
	}
}

// Find
// @Summary 题目查询 *
// @Description 题目查询（管理员）
// @Tags 题目
// @Accept json
// @Produce json
// @Param input query request.ChallengeFindRequest false "ChallengeFindRequest"
// @Router /api/challenges/ [get]
func (c *ChallengeControllerImpl) Find(ctx *gin.Context) {
	challengeData, pageCount, _ := c.challengeService.Find(request.ChallengeFindRequest{
		Title:         ctx.Query("title"),
		Category:      ctx.Query("category"),
		IsPracticable: utils.ParseIntParam(ctx.Query("is_practicable"), -1),
		IsDynamic:     utils.ParseIntParam(ctx.Query("is_dynamic"), -1),
		IsEnabled:     utils.ParseIntParam(ctx.Query("is_enabled"), -1),
		Difficulty:    utils.ParseIntParam(ctx.Query("difficulty"), -1),
		Page:          utils.ParseIntParam(ctx.Query("page"), -1),
		Size:          utils.ParseIntParam(ctx.Query("size"), -1),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"data":  challengeData,
	})
}

// Create
// @Summary 创建题目 *
// @Description 创建题目（管理员）
// @Tags 题目
// @Accept json
// @Produce json
// @Param 创建请求 body request.ChallengeCreateRequest true "ChallengeCreateRequest"
// @Router /api/challenges/ [post]
func (c *ChallengeControllerImpl) Create(ctx *gin.Context) {
	createChallengeRequest := request.ChallengeCreateRequest{}
	err := ctx.ShouldBindJSON(&createChallengeRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &createChallengeRequest),
		})
		return
	}
	_ = c.challengeService.Create(createChallengeRequest)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Update
// @Summary 更新题目 *
// @Description 更新题目（管理员）
// @Tags 题目
// @Accept json
// @Produce json
// @Param data body request.ChallengeUpdateRequest true "ChallengeUpdateRequest"
// @Router /api/challenges/ [put]
func (c *ChallengeControllerImpl) Update(ctx *gin.Context) {
	var updateChallengeRequest request.ChallengeUpdateRequest
	err := ctx.ShouldBindJSON(&updateChallengeRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &updateChallengeRequest),
		})
		return
	}
	err = c.challengeService.Update(updateChallengeRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "更新失败",
		})
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Delete
// @Summary 删除题目 *
// @Description 删除题目（管理员）
// @Tags 题目
// @Accept json
// @Produce json
// @Param data body request.ChallengeDeleteRequest true "ChallengeDeleteRequest"
// @Router /api/challenges/ [delete]
func (c *ChallengeControllerImpl) Delete(ctx *gin.Context) {
	deleteChallengeRequest := request.ChallengeDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteChallengeRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &deleteChallengeRequest),
		})
		return
	}
	err = c.challengeService.Delete(deleteChallengeRequest.ChallengeId)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "删除失败",
		})
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// FindById
// @Summary 题目查询 *
// @Description 题目查询（管理员）
// @Tags 题目
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /api/challenges/{id} [get]
func (c *ChallengeControllerImpl) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	challengeData := c.challengeService.FindById(id)
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": challengeData,
	})
}
