package repositories

import "github.com/elabosak233/pgshub/repositories/relations"

type AppRepository struct {
	UserRepository          UserRepository
	ChallengeRepository     ChallengeRepository
	TeamRepository          TeamRepository
	SubmissionRepository    SubmissionRepository
	InstanceRepository      InstanceRepository
	GameRepository          GameRepository
	UserTeamRepository      relations.UserTeamRepository
	GameChallengeRepository relations.GameChallengeRepository
}
