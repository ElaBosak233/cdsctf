package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
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
	err := ctx.ShouldBind(&submissionFindRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &submissionFindRequest),
		})
		return
	}
	submissionFindRequest.IsDetailed = ctx.GetBool("is_detailed")
	submissions, pageCount, total, _ := c.submissionService.Find(submissionFindRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"total": total,
		"data":  submissions,
	})
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
	err := ctx.ShouldBindJSON(&submissionCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &submissionCreateRequest),
		})
		return
	}
	user, _ := ctx.Get("user")
	submissionCreateRequest.UserID = user.(*response.UserResponse).ID
	status, pts, err := c.submissionService.Create(submissionCreateRequest)
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
	err := c.submissionService.Delete(deleteSubmissionRequest.SubmissionID)
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
