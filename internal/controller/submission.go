package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/cache"
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

type ISubmissionController interface {
	Find(ctx *gin.Context)
	Create(ctx *gin.Context)
	Delete(ctx *gin.Context)
}

type SubmissionController struct {
	submissionService service.ISubmissionService
}

func NewSubmissionController(appService *service.Service) ISubmissionController {
	return &SubmissionController{
		submissionService: appService.SubmissionService,
	}
}

// Find
// @Summary 提交记录查询
// @Description
// @Tags Submission
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 查找请求 query	request.SubmissionFindRequest false "SubmissionFindRequest"
// @Router /submissions/ [get]
func (c *SubmissionController) Find(ctx *gin.Context) {
	submissionFindRequest := request.SubmissionFindRequest{}
	if err := ctx.ShouldBind(&submissionFindRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &submissionFindRequest),
		})
		return
	}
	submissionFindRequest.IsDetailed = ctx.GetBool("is_detailed")
	value, exist := cache.C().Get(fmt.Sprintf("submissions:%s", utils.HashStruct(submissionFindRequest)))
	if !exist {
		submissions, total, err := c.submissionService.Find(submissionFindRequest)
		if err != nil {
			ctx.JSON(http.StatusInternalServerError, gin.H{
				"code": http.StatusInternalServerError,
				"msg":  err.Error(),
			})
			return
		}
		value = gin.H{
			"code":  http.StatusOK,
			"total": total,
			"data":  submissions,
		}
		cache.C().Set(
			fmt.Sprintf("submissions:%s", utils.HashStruct(submissionFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
}

// Create
// @Summary 提交
// @Description
// @Tags Submission
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 创建请求 body request.SubmissionCreateRequest true "SubmissionCreateRequest"
// @Router /submissions/ [post]
func (c *SubmissionController) Create(ctx *gin.Context) {
	submissionCreateRequest := request.SubmissionCreateRequest{}
	if err := ctx.ShouldBindJSON(&submissionCreateRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &submissionCreateRequest),
		})
		return
	}
	user := ctx.MustGet("user").(*model.User)
	submissionCreateRequest.UserID = user.ID
	status, rank, err := c.submissionService.Create(submissionCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("submissions")
	if status == 2 {
		cache.C().DeleteByPrefix("challenges")
		cache.C().DeleteByPrefix("game_challenges")
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":   http.StatusOK,
		"rank":   rank,
		"status": status,
	})
}

// Delete
// @Summary delete submission
// @Description
// @Tags Submission
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param 删除请求 body request.SubmissionDeleteRequest true "SubmissionDeleteRequest"
// @Router /submissions/{id} [delete]
func (c *SubmissionController) Delete(ctx *gin.Context) {
	deleteSubmissionRequest := request.SubmissionDeleteRequest{}
	deleteSubmissionRequest.SubmissionID = convertor.ToUintD(ctx.Param("id"), 0)
	if err := c.submissionService.Delete(deleteSubmissionRequest.SubmissionID); err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("submissions")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
