package controllers

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type SubmissionController interface {
	Find(ctx *gin.Context)
	Create(ctx *gin.Context)
}

type SubmissionControllerImpl struct {
	SubmissionService services.SubmissionService
}

func NewSubmissionControllerImpl(appService *services.AppService) SubmissionController {
	return &SubmissionControllerImpl{
		SubmissionService: appService.SubmissionService,
	}
}

// Find
// @Summary 提交记录查询
// @Description
// @Tags 提交
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param 查找请求 query request.SubmissionFindRequest false "SubmissionFindRequest"
// @Router /api/submissions/ [get]
func (c *SubmissionControllerImpl) Find(ctx *gin.Context) {
	isDetailed := func() int {
		if ctx.GetInt("UserRole") <= 3 && utils.ParseIntParam(ctx.Query("is_detailed"), 0) == 1 {
			return 1
		}
		return 0
	}
	if ctx.Query("id") == "" {
		submissions, pageCount, _ := c.SubmissionService.Find(request.SubmissionFindRequest{
			UserId:     ctx.Query("user_id"),
			Status:     utils.ParseIntParam(ctx.Query("status"), -1),
			TeamId:     ctx.Query("team_id"),
			GameId:     int64(utils.ParseIntParam(ctx.Query("game_id"), -1)),
			IsDetailed: isDetailed(),
			Page:       utils.ParseIntParam(ctx.Query("page"), -1),
			Size:       utils.ParseIntParam(ctx.Query("size"), -1),
		})
		ctx.JSON(http.StatusOK, gin.H{
			"code":  http.StatusOK,
			"pages": pageCount,
			"data":  submissions,
		})
	}
}

// Create
// @Summary 提交
// @Description
// @Tags 提交
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param 创建请求 body request.SubmissionCreateRequest true "SubmissionCreateRequest"
// @Router /api/submissions/ [post]
func (c *SubmissionControllerImpl) Create(ctx *gin.Context) {
	submissionCreateRequest := request.SubmissionCreateRequest{}
	err := ctx.ShouldBindJSON(&submissionCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &submissionCreateRequest),
		})
		return
	}
	submissionCreateRequest.UserId = ctx.GetString("UserId")
	status, err := c.SubmissionService.Create(submissionCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":   http.StatusOK,
		"status": status,
	})
}
