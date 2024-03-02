package service

import "github.com/elabosak233/cloudsdale/internal/repository"

var (
	s *Service = nil
)

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
	ImageService      IImageService
	FlagService       IFlagService
	HintService       IHintService
}

func S() *Service {
	return s
}

func InitService() {

	appRepository := repository.R()

	mediaService := NewMediaService()
	userService := NewUserService(appRepository)
	challengeService := NewChallengeService(appRepository)
	podService := NewPodService(appRepository)
	configService := NewConfigService(appRepository)
	teamService := NewTeamService(appRepository)
	submissionService := NewSubmissionService(appRepository)
	gameService := NewGameService(appRepository)
	categoryService := NewCategoryService(appRepository)
	containerService := NewInstanceService(appRepository)
	imageService := NewImageService(appRepository)
	flagService := NewFlagService(appRepository)
	hintService := NewHintService(appRepository)

	s = &Service{
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
		ImageService:      imageService,
		FlagService:       flagService,
		HintService:       hintService,
	}
}
