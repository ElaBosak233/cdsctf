package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils/calculate"
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
	challengeData, err := t.challengeRepository.FindById(req.ID, 1)
	if err != nil || challengeData.ID == 0 {
		return errors.New("题目不存在")
	}
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
		var cha response.ChallengeResponse
		_ = mapstructure.Decode(challenge, &cha)
		challenges = append(challenges, cha)
	}

	challengeMap := make(map[uint]response.ChallengeResponse)
	challengeIDs := make([]uint, 0)
	for _, challenge := range challenges {
		challengeMap[challenge.ID] = challenge
		challengeIDs = append(challengeIDs, challenge.ID)
	}

	gameChallengesMap := make(map[uint]model.GameChallenge)
	submissionsMap := make(map[uint][]model.Submission)
	isGame := req.GameID != nil && req.TeamID != nil
	if isGame {
		gameChallenges, _ := t.gameChallengeRepository.BatchFindByGameIdAndChallengeId(*(req.GameID), challengeIDs)
		for _, gameChallenge := range gameChallenges {
			gameChallengesMap[gameChallenge.ChallengeID] = gameChallenge
		}
		submissions, _ := t.submissionRepository.FindByChallengeID(request.SubmissionFindByChallengeIDRequest{
			GameID:      req.GameID,
			TeamID:      req.TeamID,
			Status:      2,
			ChallengeID: challengeIDs,
		})
		for _, submission := range submissions {
			submissionsMap[submission.ChallengeID] = append(submissionsMap[submission.ChallengeID], submission)
		}
	}

	// Judge isSolved
	if isGame {
		submissions, _ := t.submissionRepository.FindByChallengeID(request.SubmissionFindByChallengeIDRequest{
			GameID:      req.GameID,
			TeamID:      req.TeamID,
			Status:      2,
			ChallengeID: challengeIDs,
		})
		for _, submission := range submissions {
			challenge := challengeMap[submission.ChallengeID]
			challenge.IsSolved = true
			challengeMap[submission.ChallengeID] = challenge
		}
	} else {
		submissions, _ := t.submissionRepository.FindByChallengeID(request.SubmissionFindByChallengeIDRequest{
			UserID:      req.UserID,
			Status:      2,
			ChallengeID: challengeIDs,
		})
		for _, submission := range submissions {
			challenge := challengeMap[submission.ChallengeID]
			challenge.IsSolved = true
			challengeMap[submission.ChallengeID] = challenge
		}
	}

	for index, challenge := range challengeMap {
		// Calculate pts
		if isGame {
			challengeID := challenge.ID
			ss := gameChallengesMap[challengeID].MaxPts
			R := gameChallengesMap[challengeID].MinPts
			d := challenge.Difficulty
			x := len(submissionsMap[challengeID])
			pts := calculate.ChallengePts(ss, R, d, x)
			challenge.Pts = pts
		} else {
			challenge.Pts = challenge.PracticePts
		}

		// IsDetailed or not
		if req.IsDetailed != nil && !*(req.IsDetailed) {
			challenge.Flags = nil
		}
		challengeMap[index] = challenge
	}

	// Overwrite challenges
	for index, challenge := range challenges {
		challenges[index] = challengeMap[challenge.ID]
	}

	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return challenges, pageCount, count, err
}
