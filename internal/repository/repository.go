package repository

import (
	"github.com/elabosak233/cloudsdale/internal/database"
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
	ContainerRepository     IContainerRepository
	EnvRepository           IEnvRepository
	FlagGenRepository       IFlagGenRepository
	GameTeamRepository      IGameTeamRepository
	HintRepository          IHintRepository
	GroupRepository         IGroupRepository
	NoticeRepository        INoticeRepository
}

func R() *Repository {
	return r
}

func InitRepository() {
	onceRepository.Do(func() {
		db := database.Db()

		r = &Repository{
			UserRepository:          NewUserRepository(db),
			ChallengeRepository:     NewChallengeRepository(db),
			TeamRepository:          NewTeamRepository(db),
			SubmissionRepository:    NewSubmissionRepository(db),
			PodRepository:           NewPodRepository(db),
			GameRepository:          NewGameRepository(db),
			UserTeamRepository:      NewUserTeamRepository(db),
			GameChallengeRepository: NewGameChallengeRepository(db),
			CategoryRepository:      NewCategoryRepositoryImpl(db),
			FlagRepository:          NewFlagRepository(db),
			PortRepository:          NewPortRepository(db),
			NatRepository:           NewNatRepository(db),
			ContainerRepository:     NewContainerRepository(db),
			EnvRepository:           NewEnvRepository(db),
			FlagGenRepository:       NewFlagGenRepository(db),
			GameTeamRepository:      NewGameTeamRepository(db),
			HintRepository:          NewHintRepository(db),
			GroupRepository:         NewGroupRepository(db),
			NoticeRepository:        NewNoticeRepository(db),
		}
	})
}
