package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
	"math"
)

type IChallengeService interface {
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error)
	Create(req request.ChallengeCreateRequest) (err error)
	Update(req request.ChallengeUpdateRequest) (err error)
	Delete(id uint) (err error)
}

type ChallengeService struct {
	challengeRepository     repository.IChallengeRepository
	flagRepository          repository.IFlagRepository
	imageRepository         repository.IImageRepository
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
		imageRepository:         appRepository.ImageRepository,
		portRepository:          appRepository.PortRepository,
		envRepository:           appRepository.EnvRepository,
	}
}

func (t *ChallengeService) Create(req request.ChallengeCreateRequest) (err error) {
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	_, err = t.challengeRepository.Insert(challengeModel)

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

func (t *ChallengeService) Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error) {
	challengesData, count, err := t.challengeRepository.Find(req)

	for _, challenge := range challengesData {
		var challengeResponse response.ChallengeResponse
		_ = mapstructure.Decode(challenge, &challengeResponse)
		if !*(req.IsDetailed) {
			challengeResponse.Flags = nil
			challengeResponse.Images = nil
		}
		challenges = append(challenges, challengeResponse)
	}

	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return challenges, pageCount, count, err
}
