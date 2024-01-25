package controllers

type AppController struct {
	UserController       UserController
	ChallengeController  ChallengeController
	InstanceController   InstanceController
	ConfigController     ConfigController
	AssetController      AssetController
	TeamController       TeamController
	SubmissionController SubmissionController
	GameController       GameController
}
