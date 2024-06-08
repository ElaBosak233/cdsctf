package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/extension/cache"
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

type IChallengeController interface {
	Find(ctx *gin.Context)
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	CreateFlag(ctx *gin.Context)
	UpdateFlag(ctx *gin.Context)
	DeleteFlag(ctx *gin.Context)
	SaveAttachment(ctx *gin.Context)
	DeleteAttachment(ctx *gin.Context)
}

type ChallengeController struct {
	challengeService service.IChallengeService
	flagService      service.IFlagService
	mediaService     service.IMediaService
}

func NewChallengeController(s *service.Service) IChallengeController {
	return &ChallengeController{
		challengeService: s.ChallengeService,
		flagService:      s.FlagService,
		mediaService:     s.MediaService,
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
	isPracticable := func() *bool {
		if p, ok := ctx.Get("is_practicable"); ok {
			return p.(*bool)
		}
		return nil
	}
	challengeFindRequest := request.ChallengeFindRequest{}
	err := ctx.ShouldBindQuery(&challengeFindRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &challengeFindRequest),
		})
		return
	}
	user := ctx.MustGet("user").(*model.User)
	challengeFindRequest.UserID = user.ID
	challengeFindRequest.IsDetailed = &isDetailed
	challengeFindRequest.IsPracticable = isPracticable()
	value, exist := cache.C().Get(fmt.Sprintf("challenges:%s", utils.HashStruct(challengeFindRequest)))
	if !exist {
		challenges, total, _ := c.challengeService.Find(challengeFindRequest)
		value = gin.H{
			"code":  http.StatusOK,
			"total": total,
			"data":  challenges,
		}
		cache.C().Set(
			fmt.Sprintf("challenges:%s", utils.HashStruct(challengeFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
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
	challengeCreateRequest := request.ChallengeCreateRequest{}
	err := ctx.ShouldBindJSON(&challengeCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &challengeCreateRequest),
		})
		return
	}
	_ = c.challengeService.Create(challengeCreateRequest)
	cache.C().DeleteByPrefix("challenges")
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
	challengeUpdateRequest := request.ChallengeUpdateRequest{}
	err := ctx.ShouldBindJSON(&challengeUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &challengeUpdateRequest),
		})
		return
	}
	challengeUpdateRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.challengeService.Update(challengeUpdateRequest)
	cache.C().DeleteByPrefix("challenges")
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
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
	challengeDeleteRequest := request.ChallengeDeleteRequest{}
	challengeDeleteRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.challengeService.Delete(challengeDeleteRequest.ID)
	cache.C().DeleteByPrefix("challenges")
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
		})
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// CreateFlag
// @Summary 创建 flag
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/flags [post]
func (c *ChallengeController) CreateFlag(ctx *gin.Context) {
	flagCreateRequest := request.FlagCreateRequest{}
	if err := ctx.ShouldBindJSON(&flagCreateRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &flagCreateRequest),
		})
		return
	}
	flagCreateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.flagService.Create(flagCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("challenges")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// UpdateFlag
// @Summary 更新 flag
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/flags/{flag_id} [put]
func (c *ChallengeController) UpdateFlag(ctx *gin.Context) {
	flagUpdateRequest := request.FlagUpdateRequest{}
	err := ctx.ShouldBindJSON(&flagUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &flagUpdateRequest),
		})
		return
	}
	flagUpdateRequest.ID = convertor.ToUintD(ctx.Param("flag_id"), 0)
	flagUpdateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.flagService.Update(flagUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("challenges")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteFlag
// @Summary 删除 flag
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/flags/{flag_id} [delete]
func (c *ChallengeController) DeleteFlag(ctx *gin.Context) {
	flagDeleteRequest := request.FlagDeleteRequest{}
	flagDeleteRequest.ID = convertor.ToUintD(ctx.Param("flag_id"), 0)
	err := c.flagService.Delete(flagDeleteRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("challenges")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// SaveAttachment
// @Summary 保存附件
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param file formData file true "attachment"
// @Router /challenges/{id}/attachment [post]
func (c *ChallengeController) SaveAttachment(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	fileHeader, err := ctx.FormFile("file")
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	err = c.mediaService.SaveChallengeAttachment(id, fileHeader)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("challenges")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteAttachment
// @Summary 删除附件
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/attachment [delete]
func (c *ChallengeController) DeleteAttachment(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	err := c.mediaService.DeleteChallengeAttachment(id)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("challenges")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
