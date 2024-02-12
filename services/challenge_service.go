package services

import (
	"errors"
	"fmt"
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
	"sync"
)

type ChallengeService interface {
	Create(req request.ChallengeCreateRequest) (err error)
	Update(req request.ChallengeUpdateRequest) (err error)
	Delete(id int64) error
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error)
}

type ChallengeServiceImpl struct {
	MixinService            MixinService
	ChallengeRepository     repositories.ChallengeRepository
	FlagRepository          repositories.FlagRepository
	ImageRepository         repositories.ImageRepository
	CategoryRepository      repositories.CategoryRepository
	GameChallengeRepository relations.GameChallengeRepository
	SubmissionRepository    repositories.SubmissionRepository
	PortRepository          repositories.PortRepository
	EnvRepository           repositories.EnvRepository
}

func NewChallengeServiceImpl(appRepository *repositories.Repositories) ChallengeService {
	return &ChallengeServiceImpl{
		MixinService:            NewMixinServiceImpl(appRepository),
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

func (t *ChallengeServiceImpl) Create(req request.ChallengeCreateRequest) (err error) {
	challengeModel := entity.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	challenge, err := t.ChallengeRepository.Insert(challengeModel)

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		if req.Flags != nil {
			for _, flag := range *(req.Flags) {
				var flagModel entity.Flag
				_ = mapstructure.Decode(flag, &flagModel)
				flagModel.ChallengeID = challenge.ID
				_, _ = t.FlagRepository.Insert(flagModel)
			}
		}
		wg.Done()
	}()

	go func() {
		if req.Images != nil {
			for index := range *(req.Images) {
				(*(req.Images))[index].ChallengeID = challenge.ID
			}
			for index := range *(req.Images) {
				(*(req.Images))[index], _ = t.ImageRepository.Insert((*(req.Images))[index])
			}
			// Update Ports & Envs
			ports := make([]entity.Port, 0)
			envs := make([]entity.Env, 0)
			for _, image := range *(req.Images) {
				for _, port := range image.Ports {
					port.ImageID = image.ID
					ports = append(ports, port)
				}
				for _, env := range image.Envs {
					env.ImageID = image.ID
					envs = append(envs, env)
				}
			}
			// Don't always use batch insert. Because excessive quantity can lead to SQL execution failure.
			// Maybe 200ms is enough.
			insertedPorts := make(map[string]bool) // insertedPorts is a set, used to avoid inserting duplicate ports.
			for _, port := range ports {
				if _, ok := insertedPorts[fmt.Sprintf("%d-%d", port.ImageID, port.Value)]; !ok {
					insertedPorts[fmt.Sprintf("%d-%d", port.ImageID, port.Value)] = true
					_, err = t.PortRepository.Insert(port)
				}
			}
			insertedEnvs := make(map[string]bool)
			for _, env := range envs {
				if _, ok := insertedEnvs[fmt.Sprintf("%d-%s", env.ImageID, env.Key)]; !ok {
					insertedEnvs[fmt.Sprintf("%d-%s", env.ImageID, env.Key)] = true
					_, err = t.EnvRepository.Insert(env)
				}
			}
		}
		wg.Done()
	}()

	return err
}

func (t *ChallengeServiceImpl) Update(req request.ChallengeUpdateRequest) (err error) {
	challengeData, err := t.ChallengeRepository.FindById(req.ID, 1)
	if err != nil || challengeData.ID == 0 {
		return errors.New("题目不存在")
	}
	challengeModel := entity.Challenge{}
	_ = mapstructure.Decode(req, &challengeModel)
	challengeModel, err = t.ChallengeRepository.Update(challengeModel)

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		// Check if flag need to be changed.
		if req.Flags != nil {
			// Update Flags
			for index := range *(req.Flags) {
				(*(req.Flags))[index].ChallengeID = challengeModel.ID
			}
			_ = t.FlagRepository.DeleteByChallengeID([]int64{challengeModel.ID})
			for _, flag := range *(req.Flags) {
				_, _ = t.FlagRepository.Insert(flag)
			}
		}
		wg.Done()
	}()

	go func() {
		// Check if images and ports need to be changed.
		if req.Images != nil {
			// Necessary patch, find old images.
			oldImages, _ := t.ImageRepository.FindByChallengeID([]int64{challengeModel.ID})
			oldImageIDs := make([]int64, 0)
			for _, image := range oldImages {
				oldImageIDs = append(oldImageIDs, image.ID)
			}

			// Update Images
			for index := range *(req.Images) {
				(*(req.Images))[index].ChallengeID = challengeModel.ID
			}
			_ = t.ImageRepository.DeleteByChallengeID([]int64{challengeModel.ID})
			// Don't use batch insert. Because we need the id of the image.
			for index := range *(req.Images) {
				(*(req.Images))[index], _ = t.ImageRepository.Insert((*(req.Images))[index])
			}

			// Update Ports
			ports := make([]entity.Port, 0)
			for _, image := range *(req.Images) {
				for _, port := range image.Ports {
					port.ImageID = image.ID
					ports = append(ports, port)
				}
			}
			_ = t.PortRepository.DeleteByImageID(oldImageIDs)
			// Don't always use batch insert. Because excessive quantity can lead to SQL execution failure.
			// Maybe 200ms is enough.
			insertedPorts := make(map[int]bool) // insertedPorts is a set, used to avoid inserting duplicate ports.
			for _, port := range ports {
				if _, ok := insertedPorts[port.Value]; !ok {
					insertedPorts[port.Value] = true
					_, err = t.PortRepository.Insert(port)
				}
			}
		}
		wg.Done()
	}()

	wg.Wait()

	return err
}

func (t *ChallengeServiceImpl) Delete(id int64) error {
	err := t.ChallengeRepository.Delete(id)
	return err
}

func (t *ChallengeServiceImpl) Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error) {
	challenges, count, err := t.ChallengeRepository.Find(req)
	challenges, err = t.MixinService.MixinChallenge(challenges)

	challengeMap := make(map[int64]response.ChallengeResponse)
	challengeIDs := make([]int64, 0)
	for _, challenge := range challenges {
		challengeMap[challenge.ID] = challenge
		challengeIDs = append(challengeIDs, challenge.ID)
	}

	gameChallengesMap := make(map[int64]relationEntity.GameChallenge)
	submissionsMap := make(map[int64][]response.SubmissionResponse)
	isGame := validator.IsIdValid(req.GameID) && validator.IsIdValid(req.TeamID)
	if isGame {
		gameChallenges, _ := t.GameChallengeRepository.BatchFindByGameIdAndChallengeId(req.GameID, challengeIDs)
		for _, gameChallenge := range gameChallenges {
			gameChallengesMap[gameChallenge.ChallengeId] = gameChallenge
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
			pts := utils.CalculateChallengePts(S, R, d, x)
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
