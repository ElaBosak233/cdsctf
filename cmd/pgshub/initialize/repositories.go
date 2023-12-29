package initialize

import (
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/elabosak233/pgshub/internal/repositories/relations"
	"xorm.io/xorm"
)

func Repositories(db *xorm.Engine) *repositories.AppRepository {
	return &repositories.AppRepository{
		UserRepository:       repositories.NewUserRepositoryImpl(db),
		ChallengeRepository:  repositories.NewChallengeRepositoryImpl(db),
		TeamRepository:       repositories.NewTeamRepositoryImpl(db),
		SubmissionRepository: repositories.NewSubmissionRepositoryImpl(db),
		UserTeamRepository:   relations.NewUserTeamRepositoryImpl(db),
		InstanceRepository:   repositories.NewInstanceRepositoryImpl(db),
	}
}
