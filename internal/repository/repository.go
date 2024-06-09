package repository

import (
	"github.com/elabosak233/cloudsdale/internal/app/db"
	"go.uber.org/zap"
	"sync"
)

var (
	r              *Repository = nil
	onceRepository sync.Once
)

type Repository struct {
	UserRepository          IUserRepository
	ChallengeRepository     IChallengeRepository
	TeamRepository          ITeamRepository
	SubmissionRepository    ISubmissionRepository
	PodRepository           IPodRepository
	GameRepository          IGameRepository
	UserTeamRepository      IUserTeamRepository
	GameChallengeRepository IGameChallengeRepository
	CategoryRepository      ICategoryRepository
	FlagRepository          IFlagRepository
	PortRepository          IPortRepository
	NatRepository           INatRepository
	EnvRepository           IEnvRepository
	GameTeamRepository      IGameTeamRepository
	NoticeRepository        INoticeRepository
}

func R() *Repository {
	if r == nil {
		InitRepository()
	}
	return r
}

func InitRepository() {
	onceRepository.Do(func() {
		d := db.Db()

		r = &Repository{
			UserRepository:          NewUserRepository(d),
			ChallengeRepository:     NewChallengeRepository(d),
			TeamRepository:          NewTeamRepository(d),
			SubmissionRepository:    NewSubmissionRepository(d),
			PodRepository:           NewPodRepository(d),
			GameRepository:          NewGameRepository(d),
			UserTeamRepository:      NewUserTeamRepository(d),
			GameChallengeRepository: NewGameChallengeRepository(d),
			CategoryRepository:      NewCategoryRepository(d),
			FlagRepository:          NewFlagRepository(d),
			PortRepository:          NewPortRepository(d),
			NatRepository:           NewNatRepository(d),
			EnvRepository:           NewEnvRepository(d),
			GameTeamRepository:      NewGameTeamRepository(d),
			NoticeRepository:        NewNoticeRepository(d),
		}
	})
	zap.L().Info("Repository layer inits successfully.")
}
