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
	FlagGenRepository       IFlagGenRepository
	GameTeamRepository      IGameTeamRepository
	NoticeRepository        INoticeRepository
}

func R() *Repository {
	return r
}

func InitRepository() {
	onceRepository.Do(func() {
		db := db.Db()

		r = &Repository{
			UserRepository:          NewUserRepository(db),
			ChallengeRepository:     NewChallengeRepository(db),
			TeamRepository:          NewTeamRepository(db),
			SubmissionRepository:    NewSubmissionRepository(db),
			PodRepository:           NewPodRepository(db),
			GameRepository:          NewGameRepository(db),
			UserTeamRepository:      NewUserTeamRepository(db),
			GameChallengeRepository: NewGameChallengeRepository(db),
			CategoryRepository:      NewCategoryRepository(db),
			FlagRepository:          NewFlagRepository(db),
			PortRepository:          NewPortRepository(db),
			NatRepository:           NewNatRepository(db),
			EnvRepository:           NewEnvRepository(db),
			FlagGenRepository:       NewFlagGenRepository(db),
			GameTeamRepository:      NewGameTeamRepository(db),
			NoticeRepository:        NewNoticeRepository(db),
		}
	})
	zap.L().Info("Repository layer inits successfully.")
}
