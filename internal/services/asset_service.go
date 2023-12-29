package services

import (
	"github.com/elabosak233/pgshub/internal/repositories"
	"os"
)

type AssetService interface {
	GetUserAvatarList() (res []string, err error)
	GetTeamAvatarList() (res []string, err error)
}

type AssetServiceImpl struct{}

func NewAssetServiceImpl(appRepository *repositories.AppRepository) AssetService {
	return &AssetServiceImpl{}
}

func (a *AssetServiceImpl) GetUserAvatarList() (res []string, err error) {
	res = []string{}
	path := "./assets/users/avatar"
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
	path := "./assets/teams/avatar"
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
