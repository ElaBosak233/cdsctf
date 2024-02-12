package services

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/repositories/relations"
	"github.com/elabosak233/pgshub/utils"
	"github.com/elabosak233/pgshub/utils/convertor"
	"math"
	"regexp"
	"sync"
)

type SubmissionService interface {
	Create(req request.SubmissionCreateRequest) (status int, pts int64, err error)
	Delete(id int64) (err error)
	Find(req request.SubmissionFindRequest) (submissions []response.SubmissionResponse, pageCount int64, total int64, err error)
	BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error)
}

type SubmissionServiceImpl struct {
	MixinService            MixinService
	PodRepository           repositories.PodRepository
	SubmissionRepository    repositories.SubmissionRepository
	ChallengeRepository     repositories.ChallengeRepository
	UserRepository          repositories.UserRepository
	GameChallengeRepository relations.GameChallengeRepository
	FlagGenRepository       repositories.FlagGenRepository
}

func NewSubmissionServiceImpl(appRepository *repositories.Repositories) SubmissionService {
	return &SubmissionServiceImpl{
		MixinService:            NewMixinServiceImpl(appRepository),
		PodRepository:           appRepository.PodRepository,
		SubmissionRepository:    appRepository.SubmissionRepository,
		ChallengeRepository:     appRepository.ChallengeRepository,
		UserRepository:          appRepository.UserRepository,
		GameChallengeRepository: appRepository.GameChallengeRepository,
		FlagGenRepository:       appRepository.FlagGenRepository,
	}
}

// JudgeDynamicChallenge 动态题目判断
func (t *SubmissionServiceImpl) JudgeDynamicChallenge(req request.SubmissionCreateRequest) (status int, err error) {
	perhapsPods, _, err := t.PodRepository.Find(request.PodFindRequest{
		ChallengeID: req.ChallengeID,
		GameID:      req.GameID,
		IsAvailable: convertor.TrueP(),
	})
	status = 1
	podIDs := make([]int64, 0)
	for _, pod := range perhapsPods {
		podIDs = append(podIDs, pod.ID)
	}
	flags, err := t.FlagGenRepository.FindByPodID(podIDs)
	flagMap := make(map[int64]string)
	for _, flag := range flags {
		flagMap[flag.PodID] = flag.Flag
	}
	for _, pod := range perhapsPods {
		if req.Flag == flagMap[pod.ID] {
			if (req.UserID == pod.UserID && req.UserID != 0) || (req.TeamID == pod.TeamID && req.TeamID != 0) {
				status = 2
			} else {
				status = 3
			}
			break
		}
	}
	return status, err
}

// createSync 提交创建锁，可优化
var createSync = sync.RWMutex{}

func (t *SubmissionServiceImpl) Create(req request.SubmissionCreateRequest) (status int, pts int64, err error) {
	challenges, _, err := t.ChallengeRepository.Find(request.ChallengeFindRequest{
		IDs: []int64{req.ChallengeID},
	})
	challenges, err = t.MixinService.MixinChallenge(challenges)
	challenge := challenges[0]
	status = 1
	for _, flag := range challenge.Flags {
		switch flag.Type {
		case "static":
			if flag.Value == req.Flag {
				status = max(status, 2)
				break
			}
		case "pattern":
			re := regexp.MustCompile(flag.Value)
			if re.Match([]byte(req.Flag)) {
				status = max(status, 2)
				break
			}
		case "dynamic":
			s, _ := t.JudgeDynamicChallenge(req)
			status = max(status, s)
			break
		}
	}
	createSync.Lock()
	defer createSync.Unlock()

	// Determine duplicate submissions
	if status == 2 {
		_, n, _ := t.SubmissionRepository.Find(request.SubmissionFindRequest{
			UserID:      req.UserID,
			Status:      2,
			ChallengeID: req.ChallengeID,
			TeamID:      req.TeamID,
			GameID:      req.GameID,
		})
		if n > 0 {
			status = 4
		}
	}
	if status == 2 {
		if req.GameID != 0 && req.TeamID != 0 {
			chas, _ := t.GameChallengeRepository.BatchFindByGameIdAndChallengeId(req.GameID, []int64{req.ChallengeID})
			submissions, _, _ := t.SubmissionRepository.Find(request.SubmissionFindRequest{
				GameID:      req.GameID,
				ChallengeID: req.ChallengeID,
			})
			pts = utils.CalculateChallengePts(chas[0].MaxPts, chas[0].MinPts, challenge.Difficulty, len(submissions))
		} else {
			pts = challenge.PracticePts
		}
	}
	err = t.SubmissionRepository.Insert(entity.Submission{
		Flag:        req.Flag,
		UserID:      req.UserID,
		ChallengeID: req.ChallengeID,
		TeamID:      req.TeamID,
		GameID:      req.GameID,
		Status:      status,
		Pts:         pts,
	})
	return status, pts, err
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
	// SizePerChallenge 可以得出每个 IDs 对应的最多的 Submission
	if req.SizePerChallenge != 0 {
		submissionsPerChallenge := make(map[int64]int)
		for _, submission := range result {
			if submissionsPerChallenge[submission.ChallengeID] < req.SizePerChallenge {
				submissions = append(submissions, submission)
				submissionsPerChallenge[submission.ChallengeID] += 1
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
