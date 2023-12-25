package main

import (
	"github.com/elabosak233/pgshub/internal/repositorys"
	repositorym2m "github.com/elabosak233/pgshub/internal/repositorys/m2m"
	"xorm.io/xorm"
)

func InitRepositories(db *xorm.Engine) *repositorys.AppRepository {
	return &repositorys.AppRepository{
		UserRepository:      repositorys.NewUserRepositoryImpl(db),
		ChallengeRepository: repositorys.NewChallengeRepositoryImpl(db),
		TeamRepository:      repositorys.NewTeamRepositoryImpl(db),
		UserTeamRepository:  repositorym2m.NewUserTeamRepositoryImpl(db),
	}
}
