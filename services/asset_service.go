package services

import (
	"errors"
	"fmt"
	"github.com/elabosak233/pgshub/repositories"
	"os"
)

type AssetService interface {
	GetUserAvatarList() (res []string, err error)
	GetTeamAvatarList() (res []string, err error)
	FindChallengeAttachmentByChallengeId(id int64) (err error)
	CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error)
	DeleteChallengeAttachmentByChallengeId(id int64) (err error)
}

type AssetServiceImpl struct{}

func NewAssetServiceImpl(appRepository *repositories.Repositories) AssetService {
	return &AssetServiceImpl{}
}

func (a *AssetServiceImpl) GetUserAvatarList() (res []string, err error) {
	res = []string{}
	path := "./uploads/users/avatar"
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

func (a *AssetServiceImpl) GetTeamAvatarList() (res []string, err error) {
	res = []string{}
	path := "./uploads/teams/avatar"
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

func (a *AssetServiceImpl) FindChallengeAttachmentByChallengeId(id int64) (err error) {
	//TODO implement me
	panic("implement me")
}

func (a *AssetServiceImpl) CheckChallengeAttachmentByChallengeId(id int64) (fileName string, fileSize int64, err error) {
	path := fmt.Sprintf("./uploads/challenges/attachments/%d", id)
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

func (a *AssetServiceImpl) DeleteChallengeAttachmentByChallengeId(id int64) (err error) {
	path := fmt.Sprintf("./uploads/challenges/attachments/%d", id)
	files, err := os.ReadDir(path)
	if len(files) == 0 {
		return nil
	} else {
		err = os.RemoveAll(path)
	}
	return err
}
