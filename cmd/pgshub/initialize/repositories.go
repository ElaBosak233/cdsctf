package initialize

import (
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/repositories/relations"
	"github.com/xormplus/xorm"
)

func Repositories(db *xorm.Engine) *repositories.AppRepository {
	return &repositories.AppRepository{
		UserRepository:          repositories.NewUserRepositoryImpl(db),
		ChallengeRepository:     repositories.NewChallengeRepositoryImpl(db),
		TeamRepository:          repositories.NewTeamRepositoryImpl(db),
		SubmissionRepository:    repositories.NewSubmissionRepositoryImpl(db),
		InstanceRepository:      repositories.NewInstanceRepositoryImpl(db),
		GameRepository:          repositories.NewGameRepositoryImpl(db),
		UserTeamRepository:      relations.NewUserTeamRepositoryImpl(db),
		GameChallengeRepository: relations.NewGameChallengeRepositoryImpl(db),
	}
}
