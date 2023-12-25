package repositorys

import "github.com/elabosak233/pgshub/internal/repositorys/m2m"

type AppRepository struct {
	UserRepository      UserRepository
	ChallengeRepository ChallengeRepository
	TeamRepository      TeamRepository
	UserTeamRepository  m2m.UserTeamRepository
}
