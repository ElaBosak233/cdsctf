package services

import "github.com/elabosak233/pgshub/repositories"

type Services struct {
	AssetService      AssetService
	UserService       UserService
	ChallengeService  ChallengeService
	InstanceService   InstanceService
	ConfigService     ConfigService
	TeamService       TeamService
	SubmissionService SubmissionService
	GameService       GameService
}

func InitServices(appRepository *repositories.Repositories) *Services {
	return &Services{
		AssetService:      NewAssetServiceImpl(appRepository),
		UserService:       NewUserServiceImpl(appRepository),
		ChallengeService:  NewChallengeServiceImpl(appRepository),
		InstanceService:   NewInstanceServiceImpl(appRepository),
		ConfigService:     NewConfigServiceImpl(appRepository),
		TeamService:       NewTeamServiceImpl(appRepository),
		SubmissionService: NewSubmissionServiceImpl(appRepository),
		GameService:       NewGameServiceImpl(appRepository),
	}
}
