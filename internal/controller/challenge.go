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

type IHintController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
}

type IChallengeController interface {
	Find(ctx *gin.Context)
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	CreateHint(ctx *gin.Context)
	UpdateHint(ctx *gin.Context)
	DeleteHint(ctx *gin.Context)
	CreateFlag(ctx *gin.Context)
	UpdateFlag(ctx *gin.Context)
	DeleteFlag(ctx *gin.Context)
	CreateImage(ctx *gin.Context)
	UpdateImage(ctx *gin.Context)
	DeleteImage(ctx *gin.Context)
}

type ChallengeController struct {
	challengeService service.IChallengeService
	imageService     service.IImageService
	flagService      service.IFlagService
	hintService      service.IHintService
}

func NewChallengeController(appService *service.Service) IChallengeController {
	return &ChallengeController{
		challengeService: appService.ChallengeService,
		imageService:     appService.ImageService,
		flagService:      appService.FlagService,
		hintService:      appService.HintService,
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
		p, ok := ctx.Get("is_practicable")
		if ok {
			return p.(*bool)
		}
		return nil
	}
	user, _ := ctx.Get("user")
	challenges, pageCount, total, _ := c.challengeService.Find(request.ChallengeFindRequest{
		Title:         ctx.Query("title"),
		CategoryID:    convertor.ToUintP(ctx.Query("category_id")),
		IsPracticable: isPracticable(),
		IDs:           convertor.ToUintSliceD(ctx.QueryArray("id"), make([]uint, 0)),
		IsDynamic:     convertor.ToBoolP(ctx.Query("is_dynamic")),
		Difficulty:    convertor.ToInt64D(ctx.Query("difficulty"), 0),
		UserID:        user.(*response.UserResponse).ID,
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
	challengeCreateRequest := request.ChallengeCreateRequest{}
	err := ctx.ShouldBindJSON(&challengeCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &challengeCreateRequest),
		})
		return
	}
	_ = c.challengeService.Create(challengeCreateRequest)
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
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &challengeUpdateRequest),
		})
		return
	}
	challengeUpdateRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.challengeService.Update(challengeUpdateRequest)
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
	challengeDeleteRequest := request.ChallengeDeleteRequest{}
	challengeDeleteRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.challengeService.Delete(challengeDeleteRequest.ID)
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

// CreateHint
// @Summary 创建提示
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/hints [post]
func (c *ChallengeController) CreateHint(ctx *gin.Context) {
	hintCreateRequest := request.HintCreateRequest{}
	err := ctx.ShouldBindJSON(&hintCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &hintCreateRequest),
		})
		return
	}
	hintCreateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.hintService.Create(hintCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// UpdateHint
// @Summary 更新提示
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/hints/{hint_id} [put]
func (c *ChallengeController) UpdateHint(ctx *gin.Context) {
	hintUpdateRequest := request.HintUpdateRequest{}
	err := ctx.ShouldBindJSON(&hintUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &hintUpdateRequest),
		})
		return
	}
	hintUpdateRequest.ID = convertor.ToUintD(ctx.Param("hint_id"), 0)
	hintUpdateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.hintService.Update(hintUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteHint
// @Summary 删除提示
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/hints/{hint_id} [delete]
func (c *ChallengeController) DeleteHint(ctx *gin.Context) {
	hintDeleteRequest := request.HintDeleteRequest{}
	hintDeleteRequest.ID = convertor.ToUintD(ctx.Param("hint_id"), 0)
	err := c.hintService.Delete(hintDeleteRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
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
	err := ctx.ShouldBindJSON(&flagCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &flagCreateRequest),
		})
		return
	}
	flagCreateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.flagService.Create(flagCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
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
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &flagUpdateRequest),
		})
		return
	}
	flagUpdateRequest.ID = convertor.ToUintD(ctx.Param("flag_id"), 0)
	flagUpdateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.flagService.Update(flagUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
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
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// CreateImage
// @Summary 创建镜像
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/images [post]
func (c *ChallengeController) CreateImage(ctx *gin.Context) {
	imageCreateRequest := request.ImageCreateRequest{}
	err := ctx.ShouldBindJSON(&imageCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &imageCreateRequest),
		})
		return
	}
	imageCreateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.imageService.Create(imageCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// UpdateImage
// @Summary 更新镜像
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/images/{image_id} [put]
func (c *ChallengeController) UpdateImage(ctx *gin.Context) {
	imageUpdateRequest := request.ImageUpdateRequest{}
	err := ctx.ShouldBindJSON(&imageUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &imageUpdateRequest),
		})
		return
	}
	imageUpdateRequest.ID = convertor.ToUintD(ctx.Param("image_id"), 0)
	imageUpdateRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.imageService.Update(imageUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// DeleteImage
// @Summary 删除镜像
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /challenges/{id}/images/{image_id} [delete]
func (c *ChallengeController) DeleteImage(ctx *gin.Context) {
	imageDeleteRequest := request.ImageDeleteRequest{}
	imageDeleteRequest.ID = convertor.ToUintD(ctx.Param("image_id"), 0)
	imageDeleteRequest.ChallengeID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.imageService.Delete(imageDeleteRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
