package services

import (
	"errors"
	"github.com/elabosak233/pgshub/models/entity"
	relationEntity "github.com/elabosak233/pgshub/models/entity/relations"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/repositories/relations"
	"github.com/elabosak233/pgshub/utils"
	"github.com/elabosak233/pgshub/utils/validator"
	"github.com/mitchellh/mapstructure"
	"math"
)

type ChallengeService interface {
	Create(req request.ChallengeCreateRequest) (err error)
	Update(req request.ChallengeUpdateRequest) (err error)
	Delete(id int64) error
	FindById(id int64, isDetailed int) entity.Challenge
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error)
}

type ChallengeServiceImpl struct {
	ChallengeRepository     repositories.ChallengeRepository
	GameChallengeRepository relations.GameChallengeRepository
	SubmissionRepository    repositories.SubmissionRepository
}

func NewChallengeServiceImpl(appRepository *repositories.Repositories) ChallengeService {
	return &ChallengeServiceImpl{
		ChallengeRepository:     appRepository.ChallengeRepository,
		GameChallengeRepository: appRepository.GameChallengeRepository,
		SubmissionRepository:    appRepository.SubmissionRepository,
	}
}

func (t *ChallengeServiceImpl) Create(req request.ChallengeCreateRequest) (err error) {
	challengeModel := entity.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	err = t.ChallengeRepository.Insert(challengeModel)
	return err
}

func (t *ChallengeServiceImpl) Update(req request.ChallengeUpdateRequest) (err error) {
	challengeData, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
	if err != nil || challengeData.ChallengeID == 0 {
		return errors.New("题目不存在")
	}
	challengeModel := entity.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	err = t.ChallengeRepository.Update(challengeModel)
	return err
}

func (t *ChallengeServiceImpl) Delete(id int64) error {
	err := t.ChallengeRepository.Delete(id)
	return err
}

func (t *ChallengeServiceImpl) Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error) {
	challenges, count, err := t.ChallengeRepository.Find(req)
	gameChallengesMap := make(map[int64]relationEntity.GameChallenge)
	submissionsMap := make(map[int64][]response.SubmissionResponse)
	isGame := validator.IsIdValid(req.GameId) && validator.IsIdValid(req.TeamId)
	if isGame {
		challengeIds := make([]int64, len(challenges))
		for _, challenge := range challenges {
			challengeIds = append(challengeIds, challenge.ChallengeID)
		}
		gameChallenges, _ := t.GameChallengeRepository.BatchFindByGameIdAndChallengeId(req.GameId, challengeIds)
		for _, gameChallenge := range gameChallenges {
			gameChallengesMap[gameChallenge.ChallengeId] = gameChallenge
		}
		submissions, _ := t.SubmissionRepository.BatchFind(request.SubmissionBatchFindRequest{
			GameId:      req.GameId,
			TeamId:      req.TeamId,
			Status:      2,
			ChallengeId: challengeIds,
		})
		for _, submission := range submissions {
			submissionsMap[submission.ChallengeID] = append(submissionsMap[submission.ChallengeID], submission)
		}
	}
	// 二次处理
	for i := range challenges {
		// 非详细模式需要去除敏感信息
		if req.IsDetailed == 0 {
			challenges[i].Flag = ""
			challenges[i].FlagEnv = ""
			challenges[i].FlagFmt = ""
			challenges[i].Image = ""
		}
		if isGame {
			challengeId := challenges[i].ChallengeID
			S := gameChallengesMap[challengeId].MaxPts
			R := gameChallengesMap[challengeId].MinPts
			d := challenges[i].Difficulty
			x := len(submissionsMap[challengeId])
			pts := utils.CalculateChallengePts(S, R, d, x)
			challenges[i].Pts = pts
		} else {
			challenges[i].Pts = challenges[i].PracticePts
		}
		if challenges[i].Submission.SubmissionID != 0 {
			challenges[i].IsSolved = true
		}
	}
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return challenges, pageCount, count, err
}

// FindById implements ChallengeService
func (t *ChallengeServiceImpl) FindById(id int64, isDetailed int) entity.Challenge {
	challengeData, _ := t.ChallengeRepository.FindById(id, isDetailed)
	return challengeData
}
