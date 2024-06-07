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
	if s == nil {
		InitService()
	}
	return s
}

func InitService() {
	onceService.Do(func() {
		r := repository.R()

		s = &Service{
			AuthService:          NewAuthService(r),
			MediaService:         NewMediaService(),
			UserService:          NewUserService(r),
			ChallengeService:     NewChallengeService(r),
			PodService:           NewPodService(r),
			ConfigService:        NewConfigService(),
			TeamService:          NewTeamService(r),
			UserTeamService:      NewUserTeamService(r),
			SubmissionService:    NewSubmissionService(r),
			GameService:          NewGameService(r),
			GameChallengeService: NewGameChallengeService(r),
			GameTeamService:      NewGameTeamService(r),
			CategoryService:      NewCategoryService(r),
			FlagService:          NewFlagService(r),
			NoticeService:        NewNoticeService(r),
		}
	})
	zap.L().Info("Service layer inits successfully.")
}
