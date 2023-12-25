package main

import (
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/repositorys/implements"
	"xorm.io/xorm"
)

func InitRepositories(db *xorm.Engine) *repositorys.AppRepository {
	return &repositorys.AppRepository{
		UserRepository:      implements.NewUserRepositoryImpl(db),
		ChallengeRepository: implements.NewChallengeRepositoryImpl(db),
		TeamRepository:      implements.NewTeamRepositoryImpl(db),
		UserTeamRepository:  implements.NewUserTeamRepositoryImpl(db),
	}
}
