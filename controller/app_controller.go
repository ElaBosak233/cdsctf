package controller

import "github.com/elabosak233/pgshub/controller/m2m"

type AppController struct {
	UserController      UserController
	GroupController     GroupController
	UserGroupController m2m.UserGroupController
	ChallengeController ChallengeController
	InstanceController  InstanceController
}
