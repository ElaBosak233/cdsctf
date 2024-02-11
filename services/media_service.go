package services

import (
	"errors"
	"fmt"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/utils/config"
	"os"
)

type MediaService interface {
	GetUserAvatarList() (res []string, err error)
	GetTeamAvatarList() (res []string, err error)
	FindChallengeAttachmentByChallengeId(id int64) (err error)
	CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error)
	DeleteChallengeAttachmentByChallengeId(id int64) (err error)
}

type MediaServiceImpl struct{}

func NewMediaServiceImpl(appRepository *repositories.Repositories) MediaService {
	return &MediaServiceImpl{}
}

func (a *MediaServiceImpl) GetUserAvatarList() (res []string, err error) {
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

func (a *MediaServiceImpl) GetTeamAvatarList() (res []string, err error) {
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

func (a *MediaServiceImpl) FindChallengeAttachmentByChallengeId(id int64) (err error) {
	//TODO implement me
	panic("implement me")
}

func (a *MediaServiceImpl) CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error) {
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

func (a *MediaServiceImpl) DeleteChallengeAttachmentByChallengeId(id int64) (err error) {
	path := fmt.Sprintf("%s/challenges/attachments/%d", config.Cfg().Server.Paths.Media, id)
	files, err := os.ReadDir(path)
	if len(files) == 0 {
		return nil
	} else {
		err = os.RemoveAll(path)
	}
	return err
}
