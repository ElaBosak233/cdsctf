package service

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"os"
)

type IMediaService interface {
	FindChallengeAttachmentByChallengeId(id int64) (err error)
	CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error)
	DeleteChallengeAttachmentByChallengeId(id int64) (err error)
}

type MediaService struct{}

func NewMediaService() IMediaService {
	return &MediaService{}
}

func (a *MediaService) FindChallengeAttachmentByChallengeId(id int64) (err error) {
	//TODO implement me
	panic("implement me")
}

func (a *MediaService) CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error) {
	p := fmt.Sprintf("%s/challenges/attachments/%d", config.AppCfg().Gin.Paths.Media, id)
	files, err := os.ReadDir(p)
	if len(files) == 0 {
		return "", 0, errors.New("无附件")
	} else {
		for _, file := range files {
			if !file.IsDir() {
				fileName = file.Name()
				fileInfo, _ := file.Info()
				fileSize = fileInfo.Size()
			}
		}
		return fileName, fileSize, err
	}
}

func (a *MediaService) DeleteChallengeAttachmentByChallengeId(id int64) (err error) {
	p := fmt.Sprintf("%s/challenges/attachments/%d", config.AppCfg().Gin.Paths.Media, id)
	files, err := os.ReadDir(p)
	if len(files) == 0 {
		return nil
	} else {
		err = os.RemoveAll(p)
	}
	return err
}
