package service

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type ChallengeDeleteRequest struct {
	Id string `json:"id" binding:"required"`
}

type ChallengeService interface {
	Create(req model.Challenge) error
	Update(req model.Challenge) error
	Delete(id string) error
	FindById(id string) model.Challenge
	FindAll() []model.Challenge
}
