package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gabriel-vasile/mimetype"
	"github.com/gin-gonic/gin"
	"mime/multipart"
	"net/http"
	"os"
)

type IMediaController interface {
	GetGameCoverByGameId(ctx *gin.Context) // 获取比赛封面
	SetGameCoverByGameId(ctx *gin.Context) // 设置比赛封面
	FindGameWriteUpByTeamId(ctx *gin.Context)
	SetChallengeAttachmentByChallengeId(ctx *gin.Context)     // 设置题目附件
	DeleteChallengeAttachmentByChallengeId(ctx *gin.Context)  // 删除题目附件
	GetChallengeAttachmentByChallengeId(ctx *gin.Context)     // 获取题目附件
	GetChallengeAttachmentInfoByChallengeId(ctx *gin.Context) // 获取题目附件信息
}

type MediaController struct {
	mediaService service.IMediaService
}

func NewMediaController(appService *service.Service) IMediaController {
	return &MediaController{
		mediaService: appService.MediaService,
	}
}

// GetGameCoverByGameId
// @Summary 通过比赛 Id 获取比赛封面
// @Description 通过比赛 Id 获取比赛封面
// @Tags Media
// @Accept json
// @Produce json
// @Param id path string true "比赛 Id"
// @Router /media/games/cover/{id} [get]
func (c *MediaController) GetGameCoverByGameId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/games/cover/%s", config.AppCfg().Gin.Paths.Media, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
	}
}

// SetGameCoverByGameId
// @Summary 通过比赛 Id 设置比赛封面
// @Description 通过比赛 Id 设置比赛封面
// @Tags Media
// @Accept multipart/form-data
// @Param id path string true "比赛 Id"
// @Param avatar formData file true "封面文件"
// @Router /media/games/cover/{id} [post]
func (c *MediaController) SetGameCoverByGameId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("avatar")
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}
	mime, err := c.detectContentType(file)
	if !mime.Is("image/jpeg") && !mime.Is("image/png") && !mime.Is("image/gif") {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "格式不被允许",
		})
		return
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("%s/games/cover/%s", config.AppCfg().Gin.Paths.Media, id))
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

// FindGameWriteUpByTeamId
// @Summary 通过团队 Id 获取比赛 Writeup
// @Description 通过团队 Id 获取比赛 Writeup
// @Tags Media
// @Accept json
// @Produce json
// @Param id path string true "团队 Id"
// @Router /media/games/writeups/{id} [get]
func (c *MediaController) FindGameWriteUpByTeamId(ctx *gin.Context) {
	id := ctx.Param("id")
	path := fmt.Sprintf("%s/games/writeups/%s.pdf", config.AppCfg().Gin.Paths.Media, id)
	_, err := os.Stat(path)
	if err == nil {
		ctx.File(path)
	} else {
		ctx.Status(http.StatusNotFound)
	}
}

// SetChallengeAttachmentByChallengeId
// @Summary 通过题目 Id 设置题目附件
// @Description 通过题目 Id 设置题目附件
// @Tags Media
// @Accept multipart/form-data
// @Param id path string true "题目 Id"
// @Param attachment formData file true "附件文件"
// @Router /media/challenges/attachments/{id} [post]
func (c *MediaController) SetChallengeAttachmentByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	file, err := ctx.FormFile("attachment")
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  "无效文件",
		})
		return
	}
	if _, fileSize, _ := c.mediaService.CheckChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0))); fileSize != 0 {
		err = c.mediaService.DeleteChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusInternalServerError,
				"msg":  err.Error(),
			})
			return
		}
	}
	err = ctx.SaveUploadedFile(file, fmt.Sprintf("%s/challenges/attachments/%s/%s", config.AppCfg().Gin.Paths.Media, id, file.Filename))
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

// GetChallengeAttachmentInfoByChallengeId
// @Summary 通过题目 Id 查找题目附件信息
// @Description 通过题目 Id 查找题目附件信息
// @Tags Media
// @Accept json
// @Param id path string true "题目 Id"
// @Router /media/challenges/attachments/{id}/info [get]
func (c *MediaController) GetChallengeAttachmentInfoByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	fileName, fileSize, err := c.mediaService.CheckChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":      http.StatusOK,
		"file_name": fileName,
		"file_size": fileSize,
	})
}

// GetChallengeAttachmentByChallengeId
// @Summary 通过题目 Id 获取题目附件
// @Description 通过题目 Id 获取题目附件
// @Tags Media
// @Accept json
// @Param id path string true "题目 Id"
// @Router /media/challenges/attachments/{id} [get]
func (c *MediaController) GetChallengeAttachmentByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	fileName, _, err := c.mediaService.CheckChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusNotFound,
		})
		return
	}
	ctx.File(fmt.Sprintf("%s/challenges/attachments/%s/%s", config.AppCfg().Gin.Paths.Media, id, fileName))
}

// DeleteChallengeAttachmentByChallengeId
// @Summary 通过题目 Id 删除 题目附件
// @Description 通过题目 Id 删除题目附件
// @Tags Media
// @Accept json
// @Param id path string true "题目 Id"
// @Router /media/challenges/attachments/{id} [delete]
func (c *MediaController) DeleteChallengeAttachmentByChallengeId(ctx *gin.Context) {
	id := ctx.Param("id")
	err := c.mediaService.DeleteChallengeAttachmentByChallengeId(int64(convertor.ToIntD(id, 0)))
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

func (c *MediaController) detectContentType(file *multipart.FileHeader) (mime *mimetype.MIME, err error) {
	src, err := file.Open()
	if err != nil {
		return nil, err
	}
	defer func(src multipart.File) {
		_ = src.Close()
	}(src)
	buffer := make([]byte, 512)
	_, err = src.Read(buffer)
	if err != nil {
		return nil, err
	}
	return mimetype.Detect(buffer), nil
}
