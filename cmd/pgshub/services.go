package main

import (
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/service"
	servicem2m "github.com/elabosak233/pgshub/service/m2m"
)

func InitServices(appRepository repository.AppRepository) service.AppService {
	return service.AppService{
		UserService:      service.NewUserServiceImpl(appRepository),
		GroupService:     service.NewGroupServiceImpl(appRepository),
		ChallengeService: service.NewChallengeServiceImpl(appRepository),
		UserGroupService: servicem2m.NewUserServiceImpl(appRepository),
		InstanceService:  service.NewInstanceServiceImpl(appRepository),
	}
}
