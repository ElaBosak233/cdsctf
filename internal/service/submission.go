package service

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils/calculate"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"go.uber.org/zap"
	"math"
	"regexp"
	"sync"
)

type ISubmissionService interface {
	Create(req request.SubmissionCreateRequest) (status int, pts int64, err error)
	Delete(id uint) (err error)
	Find(req request.SubmissionFindRequest) (submissions []model.Submission, pageCount int64, total int64, err error)
}

type SubmissionService struct {
	podRepository           repository.IPodRepository
	submissionRepository    repository.ISubmissionRepository
	challengeRepository     repository.IChallengeRepository
	userRepository          repository.IUserRepository
	gameChallengeRepository repository.IGameChallengeRepository
	flagGenRepository       repository.IFlagGenRepository
}

func NewSubmissionService(appRepository *repository.Repository) ISubmissionService {
	return &SubmissionService{
		podRepository:           appRepository.PodRepository,
		submissionRepository:    appRepository.SubmissionRepository,
		challengeRepository:     appRepository.ChallengeRepository,
		userRepository:          appRepository.UserRepository,
		gameChallengeRepository: appRepository.GameChallengeRepository,
		flagGenRepository:       appRepository.FlagGenRepository,
	}
}

// JudgeDynamicChallenge 动态题目判断
func (t *SubmissionService) JudgeDynamicChallenge(req request.SubmissionCreateRequest) (status int, err error) {
	perhapsPods, _, err := t.podRepository.Find(request.PodFindRequest{
		ChallengeID: req.ChallengeID,
		GameID:      req.GameID,
		IsAvailable: convertor.TrueP(),
	})
	status = 1
	podIDs := make([]uint, 0)
	for _, pod := range perhapsPods {
		podIDs = append(podIDs, pod.ID)
	}
	zap.L().Debug(fmt.Sprintf("podIDs: %v", podIDs))
	flags, err := t.flagGenRepository.FindByPodID(podIDs)
	flagMap := make(map[uint]string)
	for _, flag := range flags {
		flagMap[flag.PodID] = flag.Flag
	}
	for _, pod := range perhapsPods {
		if req.Flag == flagMap[pod.ID] {
			if (req.UserID == pod.UserID && req.UserID != 0) || (*(req.TeamID) == pod.TeamID && req.TeamID != nil) {
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

func (t *SubmissionService) Create(req request.SubmissionCreateRequest) (status int, pts int64, err error) {
	challenges, _, err := t.challengeRepository.Find(request.ChallengeFindRequest{
		ID: req.ChallengeID,
	})
	challenge := challenges[0]
	status = 1
	zap.L().Debug(fmt.Sprintf("req.Flag: %s", req.Flag))
	for _, flag := range challenge.Flags {
		zap.L().Debug(fmt.Sprintf("flag.Type: %s", flag.Type))
		switch *(flag.Banned) {
		case true:
			switch flag.Type {
			case "static":
				if flag.Value == req.Flag {
					status = 4
				}
			case "pattern":
				re := regexp.MustCompile(flag.Value)
				if re.Match([]byte(req.Flag)) {
					status = 4
				}
			}
		case false:
			switch flag.Type {
			case "static":
				if flag.Value == req.Flag {
					status = max(status, 2)
				}
			case "pattern":
				re := regexp.MustCompile(flag.Value)
				if re.Match([]byte(req.Flag)) {
					status = max(status, 2)
				}
			case "dynamic":
				ss, _ := t.JudgeDynamicChallenge(req)
				status = max(status, ss)
			}
		}
	}
	createSync.Lock()
	defer createSync.Unlock()

	// Determine duplicate submissions
	if status == 2 {
		_, n, _ := t.submissionRepository.Find(request.SubmissionFindRequest{
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
		if req.GameID != nil && req.TeamID != nil {
			chas, _ := t.gameChallengeRepository.BatchFindByGameIdAndChallengeId(*(req.GameID), []uint{req.ChallengeID})
			submissions, _, _ := t.submissionRepository.Find(request.SubmissionFindRequest{
				GameID:      req.GameID,
				ChallengeID: req.ChallengeID,
			})
			pts = calculate.ChallengePts(chas[0].MaxPts, chas[0].MinPts, challenge.Difficulty, len(submissions))
		} else {
			pts = challenge.PracticePts
		}
	}
	var teamID uint
	if req.TeamID != nil {
		teamID = *(req.TeamID)
	}
	var gameID uint
	if req.GameID != nil {
		gameID = *(req.GameID)
	}
	err = t.submissionRepository.Insert(model.Submission{
		Flag:        req.Flag,
		UserID:      req.UserID,
		ChallengeID: req.ChallengeID,
		TeamID:      teamID,
		GameID:      gameID,
		Status:      status,
		Pts:         pts,
	})
	return status, pts, err
}

func (t *SubmissionService) Delete(id uint) (err error) {
	err = t.submissionRepository.Delete(id)
	return err
}

func (t *SubmissionService) Find(req request.SubmissionFindRequest) (submissions []model.Submission, pageCount int64, total int64, err error) {
	submissions, count, err := t.submissionRepository.Find(req)

	if !req.IsDetailed {
		for index := range submissions {
			submissions[index].Flag = ""
		}
	}

	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return submissions, pageCount, count, err
}
