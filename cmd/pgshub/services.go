package main

import (
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/services/implements"
)

func InitServices(appRepository *repositorys.AppRepository) *services.AppService {
	return &services.AppService{
		AssetService:     implements.NewAssetServiceImpl(appRepository),
		UserService:      implements.NewUserServiceImpl(appRepository),
		ChallengeService: implements.NewChallengeServiceImpl(appRepository),
		InstanceService:  implements.NewInstanceServiceImpl(appRepository),
		ConfigService:    implements.NewConfigServiceImpl(appRepository),
		TeamService:      implements.NewTeamServiceImpl(appRepository),
	}
}
