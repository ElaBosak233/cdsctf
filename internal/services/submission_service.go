package services

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/repositories"
	"math"
)

type SubmissionService interface {
	Create(req request.SubmissionCreateRequest) (status int, err error)
	Delete(id string) (err error)
	Find(req request.SubmissionFindRequest) (submissions []model.Submission, pageCount int64, err error)
}

type SubmissionServiceImpl struct {
	InstanceRepository   repositories.InstanceRepository
	SubmissionRepository repositories.SubmissionRepository
	ChallengeRepository  repositories.ChallengeRepository
}

func NewSubmissionServiceImpl(appRepository *repositories.AppRepository) SubmissionService {
	return &SubmissionServiceImpl{
		InstanceRepository:   appRepository.InstanceRepository,
		SubmissionRepository: appRepository.SubmissionRepository,
		ChallengeRepository:  appRepository.ChallengeRepository,
	}
}

func (t *SubmissionServiceImpl) JudgeDynamicChallenge(req request.SubmissionCreateRequest) (status int, err error) {
	perhapsInstances, _, err := t.InstanceRepository.Find(request.InstanceFindRequest{
		ChallengeId: req.ChallengeId,
		GameId:      req.GameId,
		IsAvailable: 1,
		Page:        -1,
		Size:        -1,
	})
	status = 0
	for _, instance := range perhapsInstances {
		if req.Flag == instance.Flag {
			if (req.UserId == instance.UserId && req.UserId != "") || (req.TeamId == instance.TeamId && req.TeamId != "") {
				status = 1
				break
			} else {
				status = 2
				break
			}
		}
	}
	return status, err
}

func (t *SubmissionServiceImpl) JudgeStaticChallenge(reqFlag string, challengeFlag string) (status int) {
	if challengeFlag == reqFlag {
		return 1
	} else {
		return 0
	}
}

func (t *SubmissionServiceImpl) Create(req request.SubmissionCreateRequest) (status int, err error) {
	challenge, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
	if err != nil {
		return 0, err
	}
	if challenge.IsDynamic {
		status, err = t.JudgeDynamicChallenge(req)
	} else {
		status = t.JudgeStaticChallenge(req.Flag, challenge.Flag)
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

func (t *SubmissionServiceImpl) Delete(id string) (err error) {
	err = t.SubmissionRepository.Delete(id)
	return err
}

func (t *SubmissionServiceImpl) Find(req request.SubmissionFindRequest) (submissions []model.Submission, pageCount int64, err error) {
	submissions, count, err := t.SubmissionRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return submissions, pageCount, err
}
