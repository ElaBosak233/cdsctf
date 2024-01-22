package services

import (
	model "github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/utils"
	"math"
	"sync"
)

type SubmissionService interface {
	Create(req request.SubmissionCreateRequest) (status int, err error)
	Delete(id int64) (err error)
	Find(req request.SubmissionFindRequest) (submissions []response.SubmissionResponse, pageCount int64, total int64, err error)
	BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error)
}

type SubmissionServiceImpl struct {
	InstanceRepository   repositories.InstanceRepository
	SubmissionRepository repositories.SubmissionRepository
	ChallengeRepository  repositories.ChallengeRepository
	UserRepository       repositories.UserRepository
}

func NewSubmissionServiceImpl(appRepository *repositories.AppRepository) SubmissionService {
	return &SubmissionServiceImpl{
		InstanceRepository:   appRepository.InstanceRepository,
		SubmissionRepository: appRepository.SubmissionRepository,
		ChallengeRepository:  appRepository.ChallengeRepository,
		UserRepository:       appRepository.UserRepository,
	}
}

// JudgeDynamicChallenge 动态题目判断
func (t *SubmissionServiceImpl) JudgeDynamicChallenge(req request.SubmissionCreateRequest) (status int, err error) {
	perhapsInstances, _, err := t.InstanceRepository.Find(request.InstanceFindRequest{
		ChallengeId: req.ChallengeId,
		GameId:      req.GameId,
		IsAvailable: 1,
		Page:        0,
		Size:        0,
	})
	status = 1
	for _, instance := range perhapsInstances {
		if req.Flag == instance.Flag {
			if (req.UserId == instance.UserId && req.UserId != 0) || (req.TeamId == instance.TeamId && req.TeamId != 0) {
				status = 2
			} else {
				status = 3
			}
			break
		}
	}
	return status, err
}

// JudgeStaticChallenge 静态题目判断
func (t *SubmissionServiceImpl) JudgeStaticChallenge(reqFlag string, challengeFlag string) (status int) {
	if challengeFlag == reqFlag {
		return 2
	} else {
		return 1
	}
}

// createSync 提交创建锁，可优化
var createSync = sync.RWMutex{}

func (t *SubmissionServiceImpl) Create(req request.SubmissionCreateRequest) (status int, err error) {
	challenge, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
	if err != nil {
		return 0, err
	}
	if challenge.IsDynamic && req.Flag != utils.GenerateFlag(challenge.FlagFmt) {
		status, err = t.JudgeDynamicChallenge(req)
	} else {
		status = max(t.JudgeStaticChallenge(req.Flag, challenge.Flag), t.JudgeStaticChallenge(req.Flag, challenge.FlagFmt))
	}
	createSync.Lock()
	defer createSync.Unlock()
	// 判断是否重复提交
	if status == 2 {
		_, n, _ := t.SubmissionRepository.Find(request.SubmissionFindRequest{
			UserId:      req.UserId,
			Status:      2,
			ChallengeId: req.ChallengeId,
			TeamId:      req.TeamId,
			GameId:      req.GameId,
		})
		if n > 0 {
			status = 4
		}
	}
	err = t.SubmissionRepository.Insert(model.Submission{
		Flag:        req.Flag,
		UserId:      req.UserId,
		ChallengeId: req.ChallengeId,
		TeamId:      req.TeamId,
		GameId:      req.GameId,
		Status:      status,
	})
	return status, err
}

func (t *SubmissionServiceImpl) Delete(id int64) (err error) {
	err = t.SubmissionRepository.Delete(id)
	return err
}

func (t *SubmissionServiceImpl) Find(req request.SubmissionFindRequest) (submissions []response.SubmissionResponse, pageCount int64, total int64, err error) {
	submissions, count, err := t.SubmissionRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return submissions, pageCount, count, err
}

func (t *SubmissionServiceImpl) BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error) {
	result, err := t.SubmissionRepository.BatchFind(req)
	// SizePerChallenge 可以得出每个 ChallengeId 对应的最多的 Submission
	if req.SizePerChallenge != 0 {
		submissionsPerChallenge := make(map[int64]int)
		for _, submission := range result {
			if submissionsPerChallenge[submission.ChallengeId] < req.SizePerChallenge {
				submissions = append(submissions, submission)
				submissionsPerChallenge[submission.ChallengeId] += 1
			}
		}
	} else {
		submissions = result
	}
	if !req.IsDetailed {
		for index, _ := range submissions {
			submissions[index].Flag = ""
		}
	}
	return submissions, err
}
