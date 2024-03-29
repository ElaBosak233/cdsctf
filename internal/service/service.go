package service

import (
	"github.com/elabosak233/cloudsdale/internal/repository"
	"sync"
)

var (
	s           *Service = nil
	onceService sync.Once
)

type Service struct {
	MediaService         IMediaService
	UserService          IUserService
	ChallengeService     IChallengeService
	PodService           IPodService
	ConfigService        IConfigService
	TeamService          ITeamService
	UserTeamService      IUserTeamService
	SubmissionService    ISubmissionService
	GameService          IGameService
	GameChallengeService IGameChallengeService
	GameTeamService      IGameTeamService
	CategoryService      ICategoryService
	ImageService         IImageService
	FlagService          IFlagService
	HintService          IHintService
	GroupService         IGroupService
}

func S() *Service {
	return s
}

func InitService() {
	onceService.Do(func() {
		appRepository := repository.R()

		mediaService := NewMediaService()
		userService := NewUserService(appRepository)
		challengeService := NewChallengeService(appRepository)
		podService := NewPodService(appRepository)
		configService := NewConfigService(appRepository)
		teamService := NewTeamService(appRepository)
		userTeamService := NewUserTeamService(appRepository)
		submissionService := NewSubmissionService(appRepository)
		gameService := NewGameService(appRepository)
		gameChallengeService := NewGameChallengeService(appRepository)
		gameTeamService := NewGameTeamService(appRepository)
		categoryService := NewCategoryService(appRepository)
		imageService := NewImageService(appRepository)
		flagService := NewFlagService(appRepository)
		hintService := NewHintService(appRepository)
		groupService := NewGroupService(appRepository)

		s = &Service{
			MediaService:         mediaService,
			UserService:          userService,
			ChallengeService:     challengeService,
			PodService:           podService,
			ConfigService:        configService,
			TeamService:          teamService,
			UserTeamService:      userTeamService,
			SubmissionService:    submissionService,
			GameService:          gameService,
			GameChallengeService: gameChallengeService,
			GameTeamService:      gameTeamService,
			CategoryService:      categoryService,
			ImageService:         imageService,
			FlagService:          flagService,
			HintService:          hintService,
			GroupService:         groupService,
		}
	})
}
