package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
)

type ChallengeRepository interface {
	Insert(user model.Challenge) error
	Update(user model.Challenge) error
	Delete(id string) error
	FindById(id string) (challenge model.Challenge, err error)
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error)
}
