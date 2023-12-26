package services

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
)

type ChallengeService interface {
	Create(req request.ChallengeCreateRequest) (err error)
	Update(req request.ChallengeUpdateRequest) (err error)
	Delete(id string) error
	FindById(id string, isDetailed int) model.Challenge
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, pageCount int64, err error)
}
