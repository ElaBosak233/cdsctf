package relations

import (
	model "github.com/elabosak233/pgshub/models/entity/relations"
	"xorm.io/xorm"
)

type GameChallengeRepository interface {
	BatchFindByGameIdAndChallengeId(gameID int64, challengeIDs []int64) (gameChallenges []model.GameChallenge, err error)
}

type GameChallengeRepositoryImpl struct {
	Db *xorm.Engine
}

func NewGameChallengeRepositoryImpl(Db *xorm.Engine) GameChallengeRepository {
	return &GameChallengeRepositoryImpl{Db: Db}
}

func (t *GameChallengeRepositoryImpl) BatchFindByGameIdAndChallengeId(gameID int64, challengeIDs []int64) (gameChallenges []model.GameChallenge, err error) {
	err = t.Db.Table("game_challenge").Where("game_id = ?", gameID).In("challenge_id", challengeIDs).Find(&gameChallenges)
	return gameChallenges, err
}
