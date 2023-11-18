package repository

import "github.com/elabosak233/pgshub/internal/model/data"

type ChallengeRepository interface {
	Insert(user data.Challenge)
	Update(user data.Challenge)
	Delete(id string)
	SelectByGameId(gameId string) []data.Challenge
	FindById(id string) (challenge data.Challenge, err error)
	FindAll() []data.Challenge
}
