package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type IChallengeService interface {
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, total int64, err error)
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

func NewChallengeService(r *repository.Repository) IChallengeService {
	return &ChallengeService{
		challengeRepository:     r.ChallengeRepository,
		gameChallengeRepository: r.GameChallengeRepository,
		submissionRepository:    r.SubmissionRepository,
		categoryRepository:      r.CategoryRepository,
		flagRepository:          r.FlagRepository,
		portRepository:          r.PortRepository,
		envRepository:           r.EnvRepository,
	}
}

func (t *ChallengeService) Create(req request.ChallengeCreateRequest) (err error) {
	challenge := model.Challenge{}
	_ = mapstructure.Decode(req, &challenge)
	_, err = t.challengeRepository.Create(challenge)
	return err
}

func (t *ChallengeService) Update(req request.ChallengeUpdateRequest) (err error) {
	challenge := model.Challenge{}
	_ = mapstructure.Decode(req, &challenge)
	challenge, err = t.challengeRepository.Update(challenge)
	return err
}

func (t *ChallengeService) Delete(id uint) (err error) {
	err = t.challengeRepository.Delete(id)
	return err
}

func (t *ChallengeService) Find(req request.ChallengeFindRequest) (challenges []model.Challenge, total int64, err error) {
	challenges, total, err = t.challengeRepository.Find(req)

	for index, challenge := range challenges {
		if !*(req.IsDetailed) {
			challenge.Simplify()
		}
		if req.SubmissionQty != 0 && challenge.Submissions != nil {
			challenge.Submissions = challenge.Submissions[:min(req.SubmissionQty, len(challenge.Submissions))]
		}
		challenges[index] = challenge
	}

	return challenges, total, err
}
