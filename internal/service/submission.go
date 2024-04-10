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
	"time"
)

type ISubmissionService interface {
	Create(req request.SubmissionCreateRequest) (status int, rank int64, err error)
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
	gameRepository          repository.IGameRepository
	noticeRepository        repository.INoticeRepository
}

func NewSubmissionService(appRepository *repository.Repository) ISubmissionService {
	return &SubmissionService{
		podRepository:           appRepository.PodRepository,
		submissionRepository:    appRepository.SubmissionRepository,
		challengeRepository:     appRepository.ChallengeRepository,
		userRepository:          appRepository.UserRepository,
		gameChallengeRepository: appRepository.GameChallengeRepository,
		flagGenRepository:       appRepository.FlagGenRepository,
		gameRepository:          appRepository.GameRepository,
		noticeRepository:        appRepository.NoticeRepository,
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
			if (req.UserID == pod.UserID && req.UserID != 0) || (req.TeamID != nil && *(req.TeamID) == pod.TeamID) {
				status = 2
			} else {
				status = 3
			}
			break
		}
	}
	return status, err
}

func (t *SubmissionService) Create(req request.SubmissionCreateRequest) (status int, rank int64, err error) {
	challenges, _, err := t.challengeRepository.Find(request.ChallengeFindRequest{
		ID: req.ChallengeID,
	})
	challenge := challenges[0]
	status = 1
	rank = 0
	var gameChallengeID *uint = nil
	for _, flag := range challenge.Flags {
		switch *(flag.Banned) {
		case true:
			switch flag.Type {
			case "static":
				if flag.Value == req.Flag {
					status = 3
				}
			case "pattern":
				re := regexp.MustCompile(flag.Value)
				if re.Match([]byte(req.Flag)) {
					status = 3
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

	if status == 2 {
		if _, n, _ := t.submissionRepository.Find(request.SubmissionFindRequest{
			UserID:      req.UserID,
			Status:      2,
			ChallengeID: req.ChallengeID,
			TeamID:      req.TeamID,
			GameID:      req.GameID,
		}); n > 0 {
			status = 4
		}
		if status == 2 {
			rank = int64(len(challenge.Submissions) + 1)
		}
		if req.GameID != nil && req.TeamID != nil {
			isEnabled := true
			gameChallenges, _ := t.gameChallengeRepository.Find(request.GameChallengeFindRequest{
				GameID:      *(req.GameID),
				ChallengeID: req.ChallengeID,
				IsEnabled:   &isEnabled,
			})
			games, _, _ := t.gameRepository.Find(request.GameFindRequest{
				ID: *(req.GameID),
			})
			if len(gameChallenges) > 0 && len(games) > 0 {
				gameChallenge := gameChallenges[0]
				game := games[0]
				if time.Now().Unix() < game.StartedAt || time.Now().Unix() > game.EndedAt {
					status = 4
				}
				rank = int64(len(gameChallenge.Challenge.Submissions) + 1)
				if rank <= 3 && rank != 0 {
					var noticeType string
					switch rank {
					case 1:
						noticeType = "first_blood"
					case 2:
						noticeType = "second_blood"
					case 3:
						noticeType = "third_blood"
					}
					_, err = t.noticeRepository.Create(model.Notice{
						Type:        noticeType,
						GameID:      req.GameID,
						UserID:      &req.UserID,
						TeamID:      req.TeamID,
						ChallengeID: &req.ChallengeID,
					})
				}
				gameChallengeID = &gameChallenge.ID
			}
			if len(gameChallenges) == 0 {
				status = 4
			}
		}
	}
	err = t.submissionRepository.Create(model.Submission{
		Flag:            req.Flag,
		UserID:          req.UserID,
		ChallengeID:     req.ChallengeID,
		GameChallengeID: gameChallengeID,
		TeamID:          req.TeamID,
		GameID:          req.GameID,
		Status:          status,
		Rank:            rank,
	})
	return status, rank, err
}

func (t *SubmissionService) Delete(id uint) (err error) {
	err = t.submissionRepository.Delete(id)
	return err
}

func (t *SubmissionService) Find(req request.SubmissionFindRequest) (submissions []model.Submission, pageCount int64, total int64, err error) {
	submissions, count, err := t.submissionRepository.Find(req)

	for index, submission := range submissions {
		if submission.Status == 2 {
			if submission.GameID != nil && submission.GameChallengeID != nil {
				submission.Pts = calculate.GameChallengePts(
					submission.GameChallenge.MaxPts,
					submission.GameChallenge.MinPts,
					submission.Challenge.Difficulty,
					int(submission.Rank-1),
					submission.Game.FirstBloodRewardRatio,
					submission.Game.SecondBloodRewardRatio,
					submission.Game.ThirdBloodRewardRatio,
				)
			} else {
				submission.Pts = submission.Challenge.PracticePts
			}
		}
		if !req.IsDetailed {
			submission.Flag = ""
		}
		submissions[index] = submission
	}

	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return submissions, pageCount, count, err
}
