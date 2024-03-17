package service

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"io"
	"mime/multipart"
	"os"
	"path"
)

type IMediaService interface {
	SetUserAvatarByUserId(id uint, file *multipart.FileHeader) (err error)
	GetUserAvatarByUserId(id uint) (dst string, err error)
	GetUserAvatarInfoByUserId(id uint) (fileName string, fileSize int64, err error)
	DeleteUserAvatarByUserId(id uint) (err error)
	FindChallengeAttachmentByChallengeId(id int64) (err error)
	CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error)
	DeleteChallengeAttachmentByChallengeId(id int64) (err error)
}

type MediaService struct{}

func NewMediaService() IMediaService {
	return &MediaService{}
}

func (a *MediaService) GetUserAvatarByUserId(id uint) (dst string, err error) {
	fileName, _, err := a.GetUserAvatarInfoByUserId(id)
	dst = path.Join(config.AppCfg().Gin.Paths.Media, "/users/avatar", fmt.Sprintf("/%d", id), fileName)
	return dst, err
}

func (a *MediaService) GetUserAvatarInfoByUserId(id uint) (fileName string, fileSize int64, err error) {
	p := path.Join(config.AppCfg().Gin.Paths.Media, "/users/avatar", fmt.Sprintf("/%d", id))
	files, err := os.ReadDir(p)
	if len(files) == 0 {
		return "", 0, errors.New("无头像")
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

func (a *MediaService) SetUserAvatarByUserId(id uint, file *multipart.FileHeader) (err error) {
	mime := file.Header.Get("Content-Type")
	if mime != "image/jpeg" && mime != "image/png" {
		return errors.New("图片格式错误")
	}
	p := path.Join(config.AppCfg().Gin.Paths.Media, "/users/avatar", fmt.Sprintf("/%d", id), file.Filename)
	err = os.MkdirAll(path.Dir(p), os.ModePerm)
	src, err := file.Open()
	defer func(src multipart.File) {
		_ = src.Close()
	}(src)
	dst, err := os.Create(p)
	defer func(dst *os.File) {
		_ = dst.Close()
	}(dst)
	_, err = io.Copy(dst, src)
	return err
}

func (a *MediaService) DeleteUserAvatarByUserId(id uint) (err error) {
	p := path.Join(config.AppCfg().Gin.Paths.Media, "/users/avatar", fmt.Sprintf("/%d", id))
	err = os.RemoveAll(p)
	return err
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
