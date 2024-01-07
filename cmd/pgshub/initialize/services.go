package initialize

import (
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/elabosak233/pgshub/internal/services"
)

func Services(appRepository *repositories.AppRepository) *services.AppService {
	return &services.AppService{
		AssetService:      services.NewAssetServiceImpl(appRepository),
		UserService:       services.NewUserServiceImpl(appRepository),
		ChallengeService:  services.NewChallengeServiceImpl(appRepository),
		InstanceService:   services.NewInstanceServiceImpl(appRepository),
		ConfigService:     services.NewConfigServiceImpl(appRepository),
		TeamService:       services.NewTeamServiceImpl(appRepository),
		SubmissionService: services.NewSubmissionServiceImpl(appRepository),
	}
}
