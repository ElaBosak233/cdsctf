package controller

import (
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ChallengeControllerImpl struct {
	challengeService service.ChallengeService
}

func NewChallengeController(appService *service.AppService) ChallengeController {
	return &ChallengeControllerImpl{
		challengeService: appService.ChallengeService,
	}
}

// Create
// @Summary 创建题目
// @Description 创建题目
// @Tags 题目
// @Accept json
// @Produce json
// @Param data body request.ChallengeCreateRequest true "ChallengeCreateRequest"
// @Router /api/challenges [post]
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
// @Summary 更新题目
// @Description 更新题目
// @Tags 题目
// @Accept json
// @Produce json
// @Param data body request.ChallengeUpdateRequest true "ChallengeUpdateRequest"
// @Router /api/challenges [put]
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
// @Summary 删除题目
// @Description 删除题目
// @Tags 题目
// @Accept json
// @Produce json
// @Param data body request.ChallengeDeleteRequest true "ChallengeDeleteRequest"
// @Router /api/challenges [delete]
func (c *ChallengeControllerImpl) Delete(ctx *gin.Context) {
	deleteChallengeRequest := request.ChallengeDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteChallengeRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &deleteChallengeRequest),
		})
		return
	}
	err = c.challengeService.Delete(deleteChallengeRequest.ChallengeId)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "删除失败",
		})
	}
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// FindById
// @Summary 题目查询
// @Description 题目查询
// @Tags 题目
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /api/challenges/{id} [get]
func (c *ChallengeControllerImpl) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	challengeData := c.challengeService.FindById(id)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": challengeData,
	})
}

// FindAll
// @Summary 题目全部查询
// @Description 题目全部查询
// @Tags 题目
// @Accept json
// @Produce json
// @Router /api/challenges [get]
func (c *ChallengeControllerImpl) FindAll(ctx *gin.Context) {
	challengeData := c.challengeService.FindAll()
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": challengeData,
	})

}
