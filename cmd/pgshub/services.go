package main

import (
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/service"
)

func InitServices(appRepository *repository.AppRepository) *service.AppService {
	return &service.AppService{
		UserService:      service.NewUserServiceImpl(appRepository),
		GroupService:     service.NewGroupServiceImpl(appRepository),
		ChallengeService: service.NewChallengeServiceImpl(appRepository),
		InstanceService:  service.NewInstanceServiceImpl(appRepository),
		ConfigService:    service.NewConfigServiceImpl(appRepository),
		TeamService:      service.NewTeamServiceImpl(appRepository),
	}
}
