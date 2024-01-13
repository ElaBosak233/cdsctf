package services

import (
	"errors"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/mitchellh/mapstructure"
	"math"
)

type ChallengeService interface {
	Create(req request.ChallengeCreateRequest) (err error)
	Update(req request.ChallengeUpdateRequest) (err error)
	Delete(id int64) error
	FindById(id int64, isDetailed int) model.Challenge
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, pageCount int64, err error)
}

type ChallengeServiceImpl struct {
	ChallengeRepository repositories.ChallengeRepository
}

func NewChallengeServiceImpl(appRepository *repositories.AppRepository) ChallengeService {
	return &ChallengeServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
	}
}

func (t *ChallengeServiceImpl) Create(req request.ChallengeCreateRequest) (err error) {
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	err = t.ChallengeRepository.Insert(challengeModel)
	return err
}

func (t *ChallengeServiceImpl) Update(req request.ChallengeUpdateRequest) (err error) {
	challengeData, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
	if err != nil || challengeData.ChallengeId == 0 {
		return errors.New("题目不存在")
	}
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	err = t.ChallengeRepository.Update(challengeModel)
	return err
}

func (t *ChallengeServiceImpl) Delete(id int64) error {
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
func (t *ChallengeServiceImpl) FindById(id int64, isDetailed int) model.Challenge {
	challengeData, _ := t.ChallengeRepository.FindById(id, isDetailed)
	return challengeData
}
