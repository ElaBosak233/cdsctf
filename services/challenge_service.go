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
	Update(req request.ChallengeUpdateRequest2) (err error)
	Delete(id int64) error
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, pageCount int64, total int64, err error)
}

type ChallengeServiceImpl struct {
	ChallengeRepository     repositories.ChallengeRepository
	FlagRepository          repositories.FlagRepository
	ImageRepository         repositories.ImageRepository
	CategoryRepository      repositories.CategoryRepository
	GameChallengeRepository relations.GameChallengeRepository
	SubmissionRepository    repositories.SubmissionRepository
	PortRepository          repositories.PortRepository
}

func NewChallengeServiceImpl(appRepository *repositories.Repositories) ChallengeService {
	return &ChallengeServiceImpl{
		ChallengeRepository:     appRepository.ChallengeRepository,
		GameChallengeRepository: appRepository.GameChallengeRepository,
		SubmissionRepository:    appRepository.SubmissionRepository,
		CategoryRepository:      appRepository.CategoryRepository,
		FlagRepository:          appRepository.FlagRepository,
		ImageRepository:         appRepository.ImageRepository,
		PortRepository:          appRepository.PortRepository,
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
				flagModel.ChallengeID = challenge.ChallengeID
				_, _ = t.FlagRepository.Insert(flagModel)
			}
		}
		wg.Done()
	}()

	go func() {
		if req.Images != nil {
			for index, _ := range *(req.Images) {
				(*(req.Images))[index].ChallengeID = challenge.ChallengeID
			}
			for index, _ := range *(req.Images) {
				(*(req.Images))[index], _ = t.ImageRepository.Insert((*(req.Images))[index])
			}
			// Update Ports
			ports := make([]entity.Port, 0)
			for _, image := range *(req.Images) {
				for _, port := range image.Ports {
					port.ImageID = image.ImageID
					ports = append(ports, port)
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
		}
		wg.Done()
	}()

	return err
}

func (t *ChallengeServiceImpl) Update(req request.ChallengeUpdateRequest2) (err error) {
	challengeData, err := t.ChallengeRepository.FindById(req.ChallengeID, 1)
	if err != nil || challengeData.ChallengeID == 0 {
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
			for index, _ := range *(req.Flags) {
				(*(req.Flags))[index].ChallengeID = challengeModel.ChallengeID
			}
			_ = t.FlagRepository.DeleteByChallengeID([]int64{challengeModel.ChallengeID})
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
			oldImages, _ := t.ImageRepository.FindByChallengeID([]int64{challengeModel.ChallengeID})
			oldImageIDs := make([]int64, 0)
			for _, image := range oldImages {
				oldImageIDs = append(oldImageIDs, image.ImageID)
			}

			// Update Images
			for index, _ := range *(req.Images) {
				(*(req.Images))[index].ChallengeID = challengeModel.ChallengeID
			}
			_ = t.ImageRepository.DeleteByChallengeID([]int64{challengeModel.ChallengeID})
			// Don't use batch insert. Because we need the id of the image.
			for index, _ := range *(req.Images) {
				(*(req.Images))[index], _ = t.ImageRepository.Insert((*(req.Images))[index])
			}

			// Update Ports
			ports := make([]entity.Port, 0)
			for _, image := range *(req.Images) {
				for _, port := range image.Ports {
					port.ImageID = image.ImageID
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

	// Init challenge map.
	challengeMap := make(map[int64]response.ChallengeResponse)
	challengeIDs := make([]int64, 0)
	for _, challenge := range challenges {
		challengeMap[challenge.ChallengeID] = challenge
		challengeIDs = append(challengeIDs, challenge.ChallengeID)
	}

	gameChallengesMap := make(map[int64]relationEntity.GameChallenge)
	submissionsMap := make(map[int64][]response.SubmissionResponse)
	isGame := validator.IsIdValid(req.GameId) && validator.IsIdValid(req.TeamId)
	if isGame {
		gameChallenges, _ := t.GameChallengeRepository.BatchFindByGameIdAndChallengeId(req.GameId, challengeIDs)
		for _, gameChallenge := range gameChallenges {
			gameChallengesMap[gameChallenge.ChallengeId] = gameChallenge
		}
		submissions, _ := t.SubmissionRepository.BatchFind(request.SubmissionBatchFindRequest{
			GameId:      req.GameId,
			TeamId:      req.TeamId,
			Status:      2,
			ChallengeId: challengeIDs,
		})
		for _, submission := range submissions {
			submissionsMap[submission.ChallengeID] = append(submissionsMap[submission.ChallengeID], submission)
		}
	}

	// Mixin category -> challenges.
	categoryIDMap := make(map[int64]bool)
	for _, challenge := range challenges {
		categoryIDMap[challenge.CategoryID] = true
	}
	categoryIDs := make([]int64, 0)
	for id, _ := range categoryIDMap {
		categoryIDs = append(categoryIDs, id)
	}
	categories, _ := t.CategoryRepository.FindByID(categoryIDs)
	for _, challenge := range challengeMap {
		for _, category := range categories {
			if challenge.CategoryID == category.CategoryID {
				challenge.Category = category
				challengeMap[challenge.ChallengeID] = challenge
				break
			}
		}
	}

	// Mixin flags -> challenges.
	flags, _ := t.FlagRepository.FindByChallengeID(challengeIDs)
	for _, flag := range flags {
		challenge := challengeMap[flag.ChallengeID]
		challenge.Flags = append(challengeMap[flag.ChallengeID].Flags, flag)
		challengeMap[flag.ChallengeID] = challenge
	}

	// Mixin images & ports -> challenges.
	images, _ := t.ImageRepository.FindByChallengeID(challengeIDs)
	imageMap := make(map[int64]entity.Image)
	imageIDs := make([]int64, 0)
	for _, image := range images {
		imageMap[image.ImageID] = image
		imageIDs = append(imageIDs, image.ImageID)
	}
	// SubMixin ports -> images.
	ports, _ := t.PortRepository.FindByImageID(imageIDs)
	for _, port := range ports {
		image := imageMap[port.ImageID]
		image.Ports = append(imageMap[port.ImageID].Ports, port)
		imageMap[image.ImageID] = image
	}
	// Mixin images -> challenges.
	for _, image := range imageMap {
		challenge := challengeMap[image.ChallengeID]
		challenge.Images = append(challengeMap[image.ChallengeID].Images, image)
		challengeMap[image.ChallengeID] = challenge
	}

	// Overwrite challenges.
	for index, challenge := range challenges {
		challenges[index] = challengeMap[challenge.ChallengeID]
	}

	for i, _ := range challenges {
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
	}
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return challenges, pageCount, count, err
}
