package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type IChallengeService interface {
	// Find will find the challenge with the given request, and return the challenges and the total number of challenges.
	Find(req request.ChallengeFindRequest) ([]model.Challenge, int64, error)

	// Create will create a new challenge with the given request.
	Create(req request.ChallengeCreateRequest) error

	// Update will update the challenge with the given request.
	Update(req request.ChallengeUpdateRequest) error

	// Delete will delete the challenge with the given request.
	Delete(id uint) error
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

func (t *ChallengeService) Create(req request.ChallengeCreateRequest) error {
	challenge := model.Challenge{}
	_ = mapstructure.Decode(req, &challenge)
	_, err := t.challengeRepository.Create(challenge)
	return err
}

func (t *ChallengeService) Update(req request.ChallengeUpdateRequest) error {
	challenge := model.Challenge{}
	_ = mapstructure.Decode(req, &challenge)
	challenge, err := t.challengeRepository.Update(challenge)
	return err
}

func (t *ChallengeService) Delete(id uint) error {
	err := t.challengeRepository.Delete(id)
	return err
}

func (t *ChallengeService) Find(req request.ChallengeFindRequest) ([]model.Challenge, int64, error) {
	challenges, total, err := t.challengeRepository.Find(req)

	for index, challenge := range challenges {
		if !*(req.IsDetailed) {
			challenge.Simplify()
		}

		// Calculate the solved times and bloods.
		challenge.SolvedTimes = len(challenge.Submissions)
		if challenge.Submissions != nil {
			challenge.Bloods = challenge.Submissions[:min(3, len(challenge.Submissions))]
		}

		challenges[index] = challenge
	}

	return challenges, total, err
}
