package repositories

import (
	"github.com/elabosak233/pgshub/repositories/relations"
	"xorm.io/xorm"
)

type Repositories struct {
	UserRepository          UserRepository
	ChallengeRepository     ChallengeRepository
	TeamRepository          TeamRepository
	SubmissionRepository    SubmissionRepository
	PodRepository           PodRepository
	GameRepository          GameRepository
	UserTeamRepository      relations.UserTeamRepository
	GameChallengeRepository relations.GameChallengeRepository
	CategoryRepository      CategoryRepository
	FlagRepository          FlagRepository
	ImageRepository         ImageRepository
	PortRepository          PortRepository
	NatRepository           NatRepository
	ContainerRepository     ContainerRepository
	EnvRepository           EnvRepository
	FlagGenRepository       FlagGenRepository
}

func InitRepositories(db *xorm.Engine) *Repositories {
	return &Repositories{
		UserRepository:          NewUserRepositoryImpl(db),
		ChallengeRepository:     NewChallengeRepositoryImpl(db),
		TeamRepository:          NewTeamRepositoryImpl(db),
		SubmissionRepository:    NewSubmissionRepositoryImpl(db),
		PodRepository:           NewPodRepositoryImpl(db),
		GameRepository:          NewGameRepositoryImpl(db),
		UserTeamRepository:      relations.NewUserTeamRepositoryImpl(db),
		GameChallengeRepository: relations.NewGameChallengeRepositoryImpl(db),
		CategoryRepository:      NewCategoryRepositoryImpl(db),
		FlagRepository:          NewFlagRepositoryImpl(db),
		ImageRepository:         NewImageRepositoryImpl(db),
		PortRepository:          NewPortRepositoryImpl(db),
		NatRepository:           NewNatRepositoryImpl(db),
		ContainerRepository:     NewContainerRepositoryImpl(db),
		EnvRepository:           NewEnvRepositoryImpl(db),
		FlagGenRepository:       NewFlagGenRepositoryImpl(db),
	}
}
