package services

import "github.com/elabosak233/pgshub/repositories"

type Services struct {
	MediaService      MediaService
	UserService       UserService
	ChallengeService  ChallengeService
	PodService        PodService
	ConfigService     ConfigService
	TeamService       TeamService
	SubmissionService SubmissionService
	GameService       GameService
	CategoryService   CategoryService
	ContainerService  ContainerService
}

func InitServices(appRepository *repositories.Repositories) *Services {

	mediaService := NewMediaServiceImpl(appRepository)
	userService := NewUserServiceImpl(appRepository)
	challengeService := NewChallengeServiceImpl(appRepository)
	podService := NewPodServiceImpl(appRepository)
	configService := NewConfigServiceImpl(appRepository)
	teamService := NewTeamServiceImpl(appRepository)
	submissionService := NewSubmissionServiceImpl(appRepository)
	gameService := NewGameServiceImpl(appRepository)
	categoryService := NewCategoryServiceImpl(appRepository)
	containerService := NewContainerServiceImpl(appRepository)

	return &Services{
		MediaService:      mediaService,
		UserService:       userService,
		ChallengeService:  challengeService,
		PodService:        podService,
		ConfigService:     configService,
		TeamService:       teamService,
		SubmissionService: submissionService,
		GameService:       gameService,
		CategoryService:   categoryService,
		ContainerService:  containerService,
	}
}
