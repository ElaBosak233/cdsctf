package service

import (
	"github.com/elabosak233/cloudsdale/internal/repository"
	"go.uber.org/zap"
	"sync"
)

var (
	s           *Service = nil
	onceService sync.Once
)

type Service struct {
	AuthService          IAuthService
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
	FlagService          IFlagService
	HintService          IHintService
	NoticeService        INoticeService
}

func S() *Service {
	return s
}

func InitService() {
	onceService.Do(func() {
		appRepository := repository.R()

		authService := NewAuthService(appRepository)
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
		flagService := NewFlagService(appRepository)
		hintService := NewHintService(appRepository)
		noticeService := NewNoticeService(appRepository)

		s = &Service{
			AuthService:          authService,
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
			FlagService:          flagService,
			HintService:          hintService,
			NoticeService:        noticeService,
		}
	})
	zap.L().Info("Services module init successfully.")
}
