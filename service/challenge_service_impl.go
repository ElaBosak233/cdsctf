package service

import (
	"errors"
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/repository"
	"github.com/google/uuid"
	"github.com/mitchellh/mapstructure"
)

type ChallengeServiceImpl struct {
	ChallengeRepository repository.ChallengeRepository
}

func NewChallengeServiceImpl(appRepository *repository.AppRepository) ChallengeService {
	return &ChallengeServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
	}
}

// Create implements UserService
func (t *ChallengeServiceImpl) Create(req request.ChallengeCreateRequest) error {
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	challengeModel.ChallengeId = uuid.NewString()
	err := t.ChallengeRepository.Insert(challengeModel)
	return err
}

// Update implements UserService
func (t *ChallengeServiceImpl) Update(req request.ChallengeUpdateRequest) error {
	challengeData, err := t.ChallengeRepository.FindById(req.ChallengeId)
	if err != nil || challengeData.ChallengeId == "" {
		return errors.New("题目不存在")
	}
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	challengeModel.ChallengeId = uuid.NewString()
	err = t.ChallengeRepository.Update(challengeModel)
	return err
}

// Delete implements UserService
func (t *ChallengeServiceImpl) Delete(id string) error {
	err := t.ChallengeRepository.Delete(id)
	return err
}

// FindAll implements UserService
func (t *ChallengeServiceImpl) FindAll() []model.Challenge {
	result := t.ChallengeRepository.FindAll()
	var challenges []model.Challenge
	for _, value := range result {
		challenges = append(challenges, value)
	}
	return challenges
}

// FindById implements UserService
func (t *ChallengeServiceImpl) FindById(id string) model.Challenge {
	challengeData, _ := t.ChallengeRepository.FindById(id)
	return challengeData
}
