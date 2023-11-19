package repository

type AppRepository struct {
	UserRepository      UserRepository
	GroupRepository     GroupRepository
	ChallengeRepository ChallengeRepository
}
