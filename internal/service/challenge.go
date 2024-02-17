package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/pkg/calculate"
	"github.com/mitchellh/mapstructure"
	"math"
)

type IChallengeService interface {
	Create(req request.ChallengeCreateRequest) (err error)
	Update(req request.ChallengeUpdateRequest) (err error)
	Delete(id uint) error
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error)
}

type ChallengeService struct {
	ChallengeRepository     repository.IChallengeRepository
	FlagRepository          repository.IFlagRepository
	ImageRepository         repository.IImageRepository
	CategoryRepository      repository.ICategoryRepository
	GameChallengeRepository repository.IGameChallengeRepository
	SubmissionRepository    repository.ISubmissionRepository
	PortRepository          repository.IPortRepository
	EnvRepository           repository.IEnvRepository
}

func NewChallengeService(appRepository *repository.Repository) IChallengeService {
	return &ChallengeService{
		ChallengeRepository:     appRepository.ChallengeRepository,
		GameChallengeRepository: appRepository.GameChallengeRepository,
		SubmissionRepository:    appRepository.SubmissionRepository,
		CategoryRepository:      appRepository.CategoryRepository,
		FlagRepository:          appRepository.FlagRepository,
		ImageRepository:         appRepository.ImageRepository,
		PortRepository:          appRepository.PortRepository,
		EnvRepository:           appRepository.EnvRepository,
	}
}

func (t *ChallengeService) Create(req request.ChallengeCreateRequest) (err error) {
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	_, err = t.ChallengeRepository.Insert(challengeModel)

	return err
}

func (t *ChallengeService) Update(req request.ChallengeUpdateRequest) (err error) {
	challengeData, err := t.ChallengeRepository.FindById(req.ID, 1)
	if err != nil || challengeData.ID == 0 {
		return errors.New("题目不存在")
	}
	challengeModel := model.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	_ = t.FlagRepository.DeleteByChallengeID([]uint{challengeModel.ID})
	_ = t.ImageRepository.DeleteByChallengeID([]uint{challengeModel.ID})
	challengeModel, err = t.ChallengeRepository.Update(challengeModel)
	return err
}

func (t *ChallengeService) Delete(id uint) error {
	err := t.ChallengeRepository.Delete(id)
	return err
}

func (t *ChallengeService) Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error) {
	challengesData, count, err := t.ChallengeRepository.Find(req)

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
	submissionsMap := make(map[uint][]response.SubmissionResponse)
	isGame := req.GameID != nil && req.TeamID != nil
	if isGame {
		gameChallenges, _ := t.GameChallengeRepository.BatchFindByGameIdAndChallengeId(*(req.GameID), challengeIDs)
		for _, gameChallenge := range gameChallenges {
			gameChallengesMap[gameChallenge.ChallengeID] = gameChallenge
		}
		submissions, _ := t.SubmissionRepository.BatchFind(request.SubmissionBatchFindRequest{
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
		submissions, _ := t.SubmissionRepository.BatchFind(request.SubmissionBatchFindRequest{
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
		submissions, _ := t.SubmissionRepository.BatchFind(request.SubmissionBatchFindRequest{
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
			S := gameChallengesMap[challengeID].MaxPts
			R := gameChallengesMap[challengeID].MinPts
			d := challenge.Difficulty
			x := len(submissionsMap[challengeID])
			pts := calculate.ChallengePts(S, R, d, x)
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
