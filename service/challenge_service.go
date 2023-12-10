package service

import (
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/model/request"
)

type ChallengeService interface {
	Create(req request.ChallengeCreateRequest) error
	Update(req request.ChallengeUpdateRequest) error
	Delete(id string) error
	FindById(id string) model.Challenge
	FindAll() []model.Challenge
}
