package repositories

import (
	"github.com/elabosak233/pgshub/internal/repositories/relations"
)

type AppRepository struct {
	UserRepository       UserRepository
	ChallengeRepository  ChallengeRepository
	TeamRepository       TeamRepository
	SubmissionRepository SubmissionRepository
	UserTeamRepository   relations.UserTeamRepository
	InstanceRepository   InstanceRepository
}
