package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
	"math"
)

type IChallengeService interface {
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, pageCount int64, total int64, err error)
	Create(req request.ChallengeCreateRequest) (err error)
	Update(req request.ChallengeUpdateRequest) (err error)
	Delete(id uint) (err error)
}

type ChallengeService struct {
	challengeRepository     repository.IChallengeRepository
	flagRepository          repository.IFlagRepository
	categoryRepository      repository.ICategoryRepository
	gameChallengeRepository repository.IGameChallengeRepository
	submissionRepository    repository.ISubmissionRepository
	portRepository          repository.IPortRepository
	envRepository           repository.IEnvRepository
}

func NewChallengeService(appRepository *repository.Repository) IChallengeService {
	return &ChallengeService{
		challengeRepository:     appRepository.ChallengeRepository,
		gameChallengeRepository: appRepository.GameChallengeRepository,
		submissionRepository:    appRepository.SubmissionRepository,
		categoryRepository:      appRepository.CategoryRepository,
		flagRepository:          appRepository.FlagRepository,
		portRepository:          appRepository.PortRepository,
		envRepository:           appRepository.EnvRepository,
	}
}

func (t *ChallengeService) Create(req request.ChallengeCreateRequest) (err error) {
	challenge := model.Challenge{}
	_ = mapstructure.Decode(req, &challenge)
	_, err = t.challengeRepository.Create(challenge)
	return err
}

func (t *ChallengeService) Update(req request.ChallengeUpdateRequest) (err error) {
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	challengeModel, err = t.challengeRepository.Update(challengeModel)
	return err
}

func (t *ChallengeService) Delete(id uint) (err error) {
	err = t.challengeRepository.Delete(id)
	return err
}

func (t *ChallengeService) Find(req request.ChallengeFindRequest) (challenges []model.Challenge, pageCount int64, total int64, err error) {
	challenges, count, err := t.challengeRepository.Find(req)

	for index, challenge := range challenges {
		if !*(req.IsDetailed) {
			challenge.Flags = nil
			challenge.ImageName = ""
		}
		if req.SubmissionQty != 0 {
			challenge.Submissions = challenge.Submissions[:min(req.SubmissionQty, len(challenge.Submissions))]
		} else {
			challenge.Submissions = nil
		}
		challenges[index] = challenge
	}

	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return challenges, pageCount, count, err
}
