package service

import "github.com/elabosak233/pgshub/service/m2m"

type AppService struct {
	GroupService     GroupService
	UserService      UserService
	ChallengeService ChallengeService
	UserGroupService m2m.UserGroupService
}
