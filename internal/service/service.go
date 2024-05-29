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
	NoticeService        INoticeService
}

func S() *Service {
	return s
}

func InitService() {
	onceService.Do(func() {
		appRepository := repository.R()

		s = &Service{
			AuthService:          NewAuthService(appRepository),
			MediaService:         NewMediaService(),
			UserService:          NewUserService(appRepository),
			ChallengeService:     NewChallengeService(appRepository),
			PodService:           NewPodService(appRepository),
			ConfigService:        NewConfigService(appRepository),
			TeamService:          NewTeamService(appRepository),
			UserTeamService:      NewUserTeamService(appRepository),
			SubmissionService:    NewSubmissionService(appRepository),
			GameService:          NewGameService(appRepository),
			GameChallengeService: NewGameChallengeService(appRepository),
			GameTeamService:      NewGameTeamService(appRepository),
			CategoryService:      NewCategoryService(appRepository),
			FlagService:          NewFlagService(appRepository),
			NoticeService:        NewNoticeService(appRepository),
		}
	})
	zap.L().Info("Service layer inits successfully.")
}
