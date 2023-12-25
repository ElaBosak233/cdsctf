package main

import (
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/services"
)

func InitServices(appRepository *repositorys.AppRepository) *services.AppService {
	return &services.AppService{
		AssetService:     services.NewAssetServiceImpl(appRepository),
		UserService:      services.NewUserServiceImpl(appRepository),
		ChallengeService: services.NewChallengeServiceImpl(appRepository),
		InstanceService:  services.NewInstanceServiceImpl(appRepository),
		ConfigService:    services.NewConfigServiceImpl(appRepository),
		TeamService:      services.NewTeamServiceImpl(appRepository),
	}
}
