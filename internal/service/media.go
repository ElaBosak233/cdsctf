package service

import (
	"errors"
	"fmt"
	"github.com/elabosak233/pgshub/internal/config"
	"github.com/elabosak233/pgshub/internal/repository"
	"os"
)

type IMediaService interface {
	GetUserAvatarList() (res []string, err error)
	GetTeamAvatarList() (res []string, err error)
	FindChallengeAttachmentByChallengeId(id int64) (err error)
	CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error)
	DeleteChallengeAttachmentByChallengeId(id int64) (err error)
}

type MediaService struct{}

func NewMediaService(appRepository *repository.Repository) IMediaService {
	return &MediaService{}
}

func (a *MediaService) GetUserAvatarList() (res []string, err error) {
	res = []string{}
	path := fmt.Sprintf("%s/users/avatar", config.Cfg().Server.Paths.Media)
	files, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}
	for _, file := range files {
		if !file.IsDir() {
			res = append(res, file.Name())
		}
	}
	return res, nil
}

func (a *MediaService) GetTeamAvatarList() (res []string, err error) {
	res = []string{}
	path := fmt.Sprintf("%s/teams/avatar", config.Cfg().Server.Paths.Media)
	files, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}
	for _, file := range files {
		if !file.IsDir() {
			res = append(res, file.Name())
		}
	}
	return res, nil
}

func (a *MediaService) FindChallengeAttachmentByChallengeId(id int64) (err error) {
	//TODO implement me
	panic("implement me")
}

func (a *MediaService) CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error) {
	path := fmt.Sprintf("%s/challenges/attachments/%d", config.Cfg().Server.Paths.Media, id)
	files, err := os.ReadDir(path)
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
	path := fmt.Sprintf("%s/challenges/attachments/%d", config.Cfg().Server.Paths.Media, id)
	files, err := os.ReadDir(path)
	if len(files) == 0 {
		return nil
	} else {
		err = os.RemoveAll(path)
	}
	return err
}
