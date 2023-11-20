package main

import (
	"github.com/elabosak233/pgshub/repository"
	repositorym2m "github.com/elabosak233/pgshub/repository/m2m"
	"xorm.io/xorm"
)

func InitRepositories(db *xorm.Engine) repository.AppRepository {
	return repository.AppRepository{
		GroupRepository:     repository.NewGroupRepositoryImpl(db),
		UserRepository:      repository.NewUserRepositoryImpl(db),
		ChallengeRepository: repository.NewChallengeRepositoryImpl(db),
		UserGroupRepository: repositorym2m.NewUserGroupRepositoryImpl(db),
	}
}
