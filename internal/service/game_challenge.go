package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils/calculate"
	"github.com/mitchellh/mapstructure"
)

type IGameChallengeService interface {
	// Find will find the challenges in game with the given request.
	Find(req request.GameChallengeFindRequest) ([]model.GameChallenge, error)

	// Create will create a new game challenge with the given request.
	Create(req request.GameChallengeCreateRequest) error

	// Update will update the game challenge with the given request.
	Update(req request.GameChallengeUpdateRequest) error

	// Delete will delete the game challenge with the given request.
	Delete(req request.GameChallengeDeleteRequest) error
}

type GameChallengeService struct {
	gameRepository          repository.IGameRepository
	gameChallengeRepository repository.IGameChallengeRepository
	noticeRepository        repository.INoticeRepository
}

func NewGameChallengeService(r *repository.Repository) IGameChallengeService {
	return &GameChallengeService{
		gameRepository:          r.GameRepository,
		gameChallengeRepository: r.GameChallengeRepository,
		noticeRepository:        r.NoticeRepository,
	}
}

func (g *GameChallengeService) Find(req request.GameChallengeFindRequest) ([]model.GameChallenge, error) {
	games, _, _ := g.gameRepository.Find(request.GameFindRequest{
		ID: req.GameID,
	})
	game := games[0]
	gameChallenges, err := g.gameChallengeRepository.Find(req)
	for i, gameChallenge := range gameChallenges {
		pts := calculate.GameChallengePts(
			gameChallenge.MaxPts,
			gameChallenge.MinPts,
			gameChallenge.Challenge.Difficulty,
			int64(len(gameChallenge.Challenge.Submissions)),
			int64(len(gameChallenge.Challenge.Submissions)),
			game.FirstBloodRewardRatio,
			game.SecondBloodRewardRatio,
			game.ThirdBloodRewardRatio,
		)
		gameChallenge.Pts = pts
		for index, submission := range gameChallenge.Challenge.Submissions {
			submission.Pts = calculate.GameChallengePts(
				gameChallenge.MaxPts,
				gameChallenge.MinPts,
				gameChallenge.Challenge.Difficulty,
				int64(len(gameChallenge.Challenge.Submissions)),
				int64(int(submission.Rank-1)),
				game.FirstBloodRewardRatio,
				game.SecondBloodRewardRatio,
				game.ThirdBloodRewardRatio,
			)
			if req.TeamID != 0 && submission.TeamID != nil && *(submission.TeamID) == req.TeamID {
				sub := submission
				gameChallenge.Challenge.Solved = sub
				break
			}
			gameChallenge.Challenge.Submissions[index] = submission
		}
		if req.SubmissionQty > 0 {
			gameChallenge.Challenge.Submissions = gameChallenge.Challenge.Submissions[:min(req.SubmissionQty, len(gameChallenge.Challenge.Submissions))]
		}
		gameChallenges[i] = gameChallenge
	}
	return gameChallenges, err
}

func (g *GameChallengeService) Create(req request.GameChallengeCreateRequest) error {
	var gameChallenge model.GameChallenge
	err := mapstructure.Decode(req, &gameChallenge)
	err = g.gameChallengeRepository.Create(gameChallenge)
	return err
}

func (g *GameChallengeService) Update(req request.GameChallengeUpdateRequest) error {
	var gameChallenge model.GameChallenge
	err := mapstructure.Decode(req, &gameChallenge)
	err = g.gameChallengeRepository.Update(gameChallenge)
	if gameChallenge.IsEnabled != nil && *(gameChallenge.IsEnabled) {
		_, err = g.noticeRepository.Create(model.Notice{
			Type:        "new_challenge",
			ChallengeID: &gameChallenge.ChallengeID,
			GameID:      &gameChallenge.GameID,
		})
	}
	return err
}

func (g *GameChallengeService) Delete(req request.GameChallengeDeleteRequest) error {
	var gameChallenge model.GameChallenge
	err := mapstructure.Decode(req, &gameChallenge)
	err = g.gameChallengeRepository.Delete(gameChallenge)
	return err
}
