package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/gabriel-vasile/mimetype"
	"github.com/gin-gonic/gin"
	"mime/multipart"
	"net/http"
	"os"
)

type IMediaController interface {
	FindGameWriteUpByTeamId(ctx *gin.Context)
}

type MediaController struct {
	mediaService service.IMediaService
}

func NewMediaController(appService *service.Service) IMediaController {
	return &MediaController{
		mediaService: appService.MediaService,
	}
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
