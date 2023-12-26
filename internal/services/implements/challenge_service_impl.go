package implements

import (
	"errors"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/google/uuid"
	"github.com/mitchellh/mapstructure"
	"math"
)

type ChallengeServiceImpl struct {
	ChallengeRepository repositorys.ChallengeRepository
}

func NewChallengeServiceImpl(appRepository *repositorys.AppRepository) services.ChallengeService {
	return &ChallengeServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
	}
}

func (t *ChallengeServiceImpl) Create(req request.ChallengeCreateRequest) (err error) {
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	challengeModel.ChallengeId = uuid.NewString()
	err = t.ChallengeRepository.Insert(challengeModel)
	return err
}

func (t *ChallengeServiceImpl) Update(req request.ChallengeUpdateRequest) (err error) {
	challengeData, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
	if err != nil || challengeData.ChallengeId == "" {
		return errors.New("题目不存在")
	}
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	err = t.ChallengeRepository.Update(challengeModel)
	return err
}

func (t *ChallengeServiceImpl) Delete(id string) error {
	err := t.ChallengeRepository.Delete(id)
	return err
}

func (t *ChallengeServiceImpl) Find(req request.ChallengeFindRequest) (challenges []model.Challenge, pageCount int64, err error) {
	challenges, count, err := t.ChallengeRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return challenges, pageCount, err
}

// FindById implements ChallengeService
func (t *ChallengeServiceImpl) FindById(id string, isDetailed int) model.Challenge {
	challengeData, _ := t.ChallengeRepository.FindById(id, isDetailed)
	return challengeData
}
