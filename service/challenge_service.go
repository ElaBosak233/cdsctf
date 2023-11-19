package service

import (
	model "github.com/elabosak233/pgshub/model/data"
	req "github.com/elabosak233/pgshub/model/request/challenge"
)

type ChallengeService interface {
	Create(req req.CreateChallengeRequest) error
	Update(req map[string]interface{}) error
	Delete(id string) error
	FindById(id string) model.Challenge
	FindAll() []model.Challenge
}
