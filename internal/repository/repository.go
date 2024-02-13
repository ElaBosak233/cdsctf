package repository

import (
	"xorm.io/xorm"
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
	ImageRepository         IImageRepository
	PortRepository          IPortRepository
	NatRepository           INatRepository
	ContainerRepository     IInstanceRepository
	EnvRepository           IEnvRepository
	FlagGenRepository       IFlagGenRepository
}

func InitRepository(db *xorm.Engine) *Repository {
	return &Repository{
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
		ImageRepository:         NewImageRepository(db),
		PortRepository:          NewPortRepository(db),
		NatRepository:           NewNatRepository(db),
		ContainerRepository:     NewInstanceRepository(db),
		EnvRepository:           NewEnvRepository(db),
		FlagGenRepository:       NewFlagGenRepository(db),
	}
}
