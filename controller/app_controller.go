package controller

type AppController struct {
	UserController      UserController
	GroupController     GroupController
	ChallengeController ChallengeController
	InstanceController  InstanceController
	ConfigController    ConfigController
	AssetController     AssetController
	TeamController      TeamController
}
