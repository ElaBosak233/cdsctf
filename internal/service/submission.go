package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils/calculate"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"regexp"
	"time"
)

type ISubmissionService interface {
	Create(req request.SubmissionCreateRequest) (status int, rank int64, err error)
	Delete(id uint) (err error)
	Find(req request.SubmissionFindRequest) (submissions []model.Submission, total int64, err error)
}

type SubmissionService struct {
	podRepository           repository.IPodRepository
	submissionRepository    repository.ISubmissionRepository
	challengeRepository     repository.IChallengeRepository
	teamRepository          repository.ITeamRepository
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
		teamRepository:          appRepository.TeamRepository,
		userRepository:          appRepository.UserRepository,
		gameChallengeRepository: appRepository.GameChallengeRepository,
		flagGenRepository:       appRepository.FlagGenRepository,
		gameRepository:          appRepository.GameRepository,
		noticeRepository:        appRepository.NoticeRepository,
	}
}

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
	flags, err := t.flagGenRepository.FindByPodID(podIDs)
	flagMap := make(map[uint]string)
	for _, flag := range flags {
		flagMap[flag.PodID] = flag.Flag
	}
	for _, pod := range perhapsPods {
		if req.Flag == flagMap[pod.ID] {
			if (pod.UserID != nil && req.UserID == *(pod.UserID) && req.UserID != 0) || (pod.TeamID != nil && req.TeamID != nil && *(req.TeamID) == *(pod.TeamID)) {
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
	var challenge model.Challenge
	if challenges, total, _ := t.challengeRepository.Find(request.ChallengeFindRequest{
		ID: req.ChallengeID,
	}); total > 0 {
		challenge = challenges[0]
	}

	var team model.Team
	if req.TeamID != nil {
		if teams, total, _ := t.teamRepository.Find(request.TeamFindRequest{
			ID: *(req.TeamID),
		}); total > 0 {
			team = teams[0]
		}
		isMember := false
		for _, user := range team.Users {
			if req.UserID == user.ID {
				isMember = true
			}
		}
		if !isMember {
			status = 4
		}
	}

	status = 1
	rank = 0
	var gameChallengeID *uint = nil
	for _, flag := range challenge.Flags {
		switch *(flag.Banned) {
		case true:
			re, regexErr := regexp.Compile(flag.Value)
			if regexErr != nil {
				return 0, 0, regexErr
			}
			if re != nil && re.Match([]byte(req.Flag)) {
				status = 3
			}
		case false:
			switch flag.Type {
			case "pattern":
				re, regexErr := regexp.Compile(flag.Value)
				if regexErr != nil {
					return 0, 0, regexErr
				}
				if re != nil && re.Match([]byte(req.Flag)) {
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
				if rank <= 3 && rank != 0 && status == 2 {
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

func (t *SubmissionService) Find(req request.SubmissionFindRequest) (submissions []model.Submission, total int64, err error) {
	submissions, total, err = t.submissionRepository.Find(req)
	challengeSolvesTotal := make(map[uint]int64)

	extractChallengeTotal := func(challengeID uint) int64 {
		var cTotal int64
		if _, ok := challengeSolvesTotal[challengeID]; !ok {
			for _, submission := range submissions {
				if submission.ChallengeID == challengeID && submission.Status == 2 {
					cTotal++
				}
			}
			challengeSolvesTotal[challengeID] = cTotal
		} else {
			cTotal = challengeSolvesTotal[challengeID]
		}
		return cTotal
	}

	for index, submission := range submissions {
		if submission.Status == 2 {
			if submission.GameID != nil && submission.GameChallengeID != nil {
				submission.Pts = calculate.GameChallengePts(
					submission.GameChallenge.MaxPts,
					submission.GameChallenge.MinPts,
					submission.Challenge.Difficulty,
					extractChallengeTotal(submission.ChallengeID),
					submission.Rank-1,
					submission.Game.FirstBloodRewardRatio,
					submission.Game.SecondBloodRewardRatio,
					submission.Game.ThirdBloodRewardRatio,
				)
			} else {
				submission.Pts = submission.Challenge.PracticePts
			}
		}
		if !req.IsDetailed {
			submission.Simplify()
		}
		submissions[index] = submission
	}
	return submissions, total, err
}
