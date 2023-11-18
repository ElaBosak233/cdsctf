package repository

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type ChallengeRepository interface {
	Insert(user model.Challenge)
	Update(user model.Challenge)
	Delete(id string)
	SelectByGameId(gameId string) []model.Challenge
	FindById(id string) (challenge model.Challenge, err error)
	FindAll() []model.Challenge
}
