package service

type AppService struct {
	GroupService     GroupService
	UserService      UserService
	ChallengeService ChallengeService
}
