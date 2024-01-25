package services

type AppService struct {
	AssetService      AssetService
	UserService       UserService
	ChallengeService  ChallengeService
	InstanceService   InstanceService
	ConfigService     ConfigService
	TeamService       TeamService
	SubmissionService SubmissionService
	GameService       GameService
}
