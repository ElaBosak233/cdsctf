package relations

import (
	model "github.com/elabosak233/pgshub/models/entity/relations"
	"github.com/xormplus/xorm"
)

type GameChallengeRepository interface {
	BatchFindByGameIdAndChallengeId(gameId int64, challengeIds []int64) (gameChallenges []model.GameChallenge, err error)
}

type GameChallengeRepositoryImpl struct {
	Db *xorm.Engine
}

func NewGameChallengeRepositoryImpl(Db *xorm.Engine) GameChallengeRepository {
	return &GameChallengeRepositoryImpl{Db: Db}
}

func (t *GameChallengeRepositoryImpl) BatchFindByGameIdAndChallengeId(gameId int64, challengeIds []int64) (gameChallenges []model.GameChallenge, err error) {
	err = t.Db.Table("game_challenge").Where("game_id = ?", gameId).In("challenge_id", challengeIds).Find(&gameChallenges)
	return gameChallenges, err
}
