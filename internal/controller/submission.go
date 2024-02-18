package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ISubmissionController interface {
	Find(ctx *gin.Context)
	Create(ctx *gin.Context)
	BatchFind(ctx *gin.Context)
	Delete(ctx *gin.Context)
}

type SubmissionController struct {
	SubmissionService service.ISubmissionService
}

func NewSubmissionController(appService *service.Service) ISubmissionController {
	return &SubmissionController{
		SubmissionService: appService.SubmissionService,
	}
}

// Find
// @Summary 提交记录查询
// @Description
// @Tags 提交
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param 查找请求 query request.SubmissionFindRequest false "SubmissionFindRequest"
// @Router /api/submissions/ [get]
func (c *SubmissionController) Find(ctx *gin.Context) {
	isDetailed := func() int {
		if ctx.GetInt64("UserLevel") <= 3 && convertor.ToIntD(ctx.Query("is_detailed"), 0) == 1 {
			return 1
		}
		return 0
	}
	submissions, pageCount, total, _ := c.SubmissionService.Find(request.SubmissionFindRequest{
		UserID:      convertor.ToUintD(ctx.Query("user_id"), 0),
		Status:      convertor.ToIntD(ctx.Query("status"), 0),
		TeamID:      convertor.ToUintP(ctx.Query("team_id")),
		GameID:      convertor.ToUintP(ctx.Query("game_id")),
		IsDetailed:  isDetailed(),
		ChallengeID: convertor.ToUintD(ctx.Query("challenge_id"), 0),
		SortBy:      ctx.QueryArray("sort_by"),
		Page:        convertor.ToIntD(ctx.Query("page"), 0),
		Size:        convertor.ToIntD(ctx.Query("size"), 0),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"total": total,
		"data":  submissions,
	})
}

// BatchFind
// @Summary 提交记录批量查询
// @Description
// @Tags 提交
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param 查找请求 query request.SubmissionBatchFindRequest false "SubmissionBatchFindRequest"
// @Router /api/submissions/batch/ [get]
func (c *SubmissionController) BatchFind(ctx *gin.Context) {
	submissions, err := c.SubmissionService.BatchFind(request.SubmissionBatchFindRequest{
		Size:             convertor.ToIntD(ctx.Query("size"), 1),
		SizePerChallenge: convertor.ToIntD(ctx.Query("size_per_challenge"), 0),
		UserID:           convertor.ToUintD(ctx.Query("user_id"), 0),
		ChallengeID:      convertor.ToUintSliceD(ctx.QueryArray("challenge_id"), []uint{}),
		Status:           convertor.ToIntD(ctx.Query("status"), 0),
		SortBy:           ctx.QueryArray("sort_by"),
		IsDetailed:       ctx.Query("is_detailed") == "true",
		TeamID:           convertor.ToUintP(ctx.Query("team_id")),
		GameID:           convertor.ToUintP(ctx.Query("game_id")),
	})
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": submissions,
	})
}

// Create
// @Summary 提交
// @Description
// @Tags 提交
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param 创建请求 body request.SubmissionCreateRequest true "SubmissionCreateRequest"
// @Router /api/submissions/ [post]
func (c *SubmissionController) Create(ctx *gin.Context) {
	submissionCreateRequest := request.SubmissionCreateRequest{}
	err := ctx.ShouldBindJSON(&submissionCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &submissionCreateRequest),
		})
		return
	}
	submissionCreateRequest.UserID = ctx.GetUint("UserID")
	status, pts, err := c.SubmissionService.Create(submissionCreateRequest)
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
		"pts":    pts,
	})
}

func (c *SubmissionController) Delete(ctx *gin.Context) {
	deleteSubmissionRequest := request.SubmissionDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteSubmissionRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &deleteSubmissionRequest),
		})
		return
	}
	err = c.SubmissionService.Delete(deleteSubmissionRequest.SubmissionID)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
