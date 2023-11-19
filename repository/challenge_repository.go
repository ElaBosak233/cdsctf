package repository

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type ChallengeRepository interface {
	Insert(user model.Challenge) error
	Update(user model.Challenge) error
	Delete(id string) error
	FindById(id string) (challenge model.Challenge, err error)
	FindAll() []model.Challenge
}
