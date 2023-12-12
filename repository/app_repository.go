package repository

import "github.com/elabosak233/pgshub/repository/m2m"

type AppRepository struct {
	UserRepository      UserRepository
	GroupRepository     GroupRepository
	ChallengeRepository ChallengeRepository
	UserGroupRepository m2m.UserGroupRepository
	TeamRepository      TeamRepository
	UserTeamRepository  m2m.UserTeamRepository
}
