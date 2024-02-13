package service

import "github.com/elabosak233/pgshub/internal/repository"

type Service struct {
	MediaService      IMediaService
	UserService       IUserService
	ChallengeService  IChallengeService
	PodService        IPodService
	ConfigService     IConfigService
	TeamService       ITeamService
	SubmissionService ISubmissionService
	GameService       IGameService
	CategoryService   ICategoryService
	ContainerService  IInstanceService
}

func InitService(appRepository *repository.Repository) *Service {

	mediaService := NewMediaService(appRepository)
	userService := NewUserService(appRepository)
	challengeService := NewChallengeService(appRepository)
	podService := NewPodService(appRepository)
	configService := NewConfigService(appRepository)
	teamService := NewTeamService(appRepository)
	submissionService := NewSubmissionService(appRepository)
	gameService := NewGameService(appRepository)
	categoryService := NewCategoryService(appRepository)
	containerService := NewInstanceService(appRepository)

	return &Service{
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
