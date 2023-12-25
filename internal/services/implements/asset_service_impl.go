package implements

import (
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/services"
	"os"
)

type AssetServiceImpl struct {
}

func NewAssetServiceImpl(appRepository *repositorys.AppRepository) services.AssetService {
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
