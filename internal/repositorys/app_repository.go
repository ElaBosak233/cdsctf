package repositorys

import "github.com/elabosak233/pgshub/internal/repositorys/relations"

type AppRepository struct {
	UserRepository      UserRepository
	ChallengeRepository ChallengeRepository
	TeamRepository      TeamRepository
	UserTeamRepository  relations.UserTeamRepository
}
