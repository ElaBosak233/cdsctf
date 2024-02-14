package service

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	"github.com/elabosak233/pgshub/internal/repository"
)

// IMixinService is a service that mixes structure to structure.
// It will be only used in other services.
type IMixinService interface {
	MixinChallenge(challenges []response.ChallengeResponse) (chas []response.ChallengeResponse, err error)
	MixinImage(images []model.Image) (imgs []model.Image, err error)
	MixinInstance(containers []model.Instance) (ctns []model.Instance, err error)
	MixinPod(pods []model.Pod) (p []model.Pod, err error)
}

type MixinService struct {
	ChallengeRepository repository.IChallengeRepository
	ImageRepository     repository.IImageRepository
	EnvRepository       repository.IEnvRepository
	PortRepository      repository.IPortRepository
	CategoryRepository  repository.ICategoryRepository
	FlagRepository      repository.IFlagRepository
	NatRepository       repository.INatRepository
	ContainerRepository repository.IInstanceRepository
}

func NewMixinService(appRepository *repository.Repository) IMixinService {
	return &MixinService{
		ChallengeRepository: appRepository.ChallengeRepository,
		ImageRepository:     appRepository.ImageRepository,
		EnvRepository:       appRepository.EnvRepository,
		PortRepository:      appRepository.PortRepository,
		CategoryRepository:  appRepository.CategoryRepository,
		FlagRepository:      appRepository.FlagRepository,
		NatRepository:       appRepository.NatRepository,
		ContainerRepository: appRepository.ContainerRepository,
	}
}

func (t *MixinService) MixinImage(images []model.Image) (imgs []model.Image, err error) {
	imageMap := make(map[int64]model.Image)
	for _, image := range images {
		imageMap[image.ID] = image
	}
	imageIDs := make([]int64, 0)
	for id := range imageMap {
		imageIDs = append(imageIDs, id)
	}
	// mixin env -> image
	envs, _ := t.EnvRepository.FindByImageID(imageIDs)
	for _, env := range envs {
		image := imageMap[env.ImageID]
		image.Envs = append(image.Envs, env)
		imageMap[env.ImageID] = image
	}

	// mixin port -> image
	ports, _ := t.PortRepository.FindByImageID(imageIDs)
	for _, port := range ports {
		image := imageMap[port.ImageID]
		image.Ports = append(image.Ports, port)
		imageMap[port.ImageID] = image
	}

	for _, image := range imageMap {
		imgs = append(imgs, image)
	}

	return imgs, err
}

func (t *MixinService) MixinChallenge(challenges []response.ChallengeResponse) (chas []response.ChallengeResponse, err error) {
	challengeMap := make(map[int64]response.ChallengeResponse)
	challengeIDs := make([]int64, 0)
	for _, challenge := range challenges {
		challengeMap[challenge.ID] = challenge
		challengeIDs = append(challengeIDs, challenge.ID)
	}
	// mixin category -> challenges
	categoryIDMap := make(map[int64]bool)
	for _, challenge := range challenges {
		categoryIDMap[challenge.CategoryID] = true
	}
	categoryIDs := make([]int64, 0)
	for id := range categoryIDMap {
		categoryIDs = append(categoryIDs, id)
	}
	categories, _ := t.CategoryRepository.FindByID(categoryIDs)
	for _, challenge := range challengeMap {
		for _, category := range categories {
			if challenge.CategoryID == category.ID {
				cate := category
				challenge.Category = &cate
				challengeMap[challenge.ID] = challenge
				break
			}
		}
	}

	// mixin flags -> challenges
	flags, _ := t.FlagRepository.FindByChallengeID(challengeIDs)
	for _, flag := range flags {
		challenge := challengeMap[flag.ChallengeID]
		challenge.Flags = append(challengeMap[flag.ChallengeID].Flags, flag)
		challengeMap[flag.ChallengeID] = challenge
	}

	// mixin images -> challenges
	images, _ := t.ImageRepository.FindByChallengeID(challengeIDs)
	images, _ = t.MixinImage(images)
	for _, image := range images {
		challenge := challengeMap[image.ChallengeID]
		challenge.Images = append(challengeMap[image.ChallengeID].Images, image)
		challengeMap[image.ChallengeID] = challenge
	}

	for _, challenge := range challenges {
		chas = append(chas, challengeMap[challenge.ID])
	}

	return chas, err
}

func (t *MixinService) MixinInstance(containers []model.Instance) (ctns []model.Instance, err error) {
	ctnMap := make(map[int64]model.Instance)
	ctnIDs := make([]int64, 0)

	imageIDMap := make(map[int64]bool)

	for _, container := range containers {
		ctnMap[container.ID] = container
		ctnIDs = append(ctnIDs, container.ID)
		imageIDMap[container.ImageID] = true
	}

	imageMap := make(map[int64]model.Image)
	imageIDs := make([]int64, 0)
	for imageID := range imageIDMap {
		imageIDs = append(imageIDs, imageID)
	}

	images, err := t.ImageRepository.FindByID(imageIDs)
	images, err = t.MixinImage(images)

	for _, image := range images {
		imageMap[image.ID] = image
	}

	// mixin image -> instance
	for index, ctn := range ctnMap {
		image := imageMap[ctn.ImageID]
		ctn.Image = &image
		ctnMap[index] = ctn
	}

	// mixin nat -> instance
	nats, _ := t.NatRepository.FindByInstanceID(ctnIDs)
	for _, nat := range nats {
		ctn := ctnMap[nat.InstanceID]
		ctn.Nats = append(ctn.Nats, nat)
		ctnMap[nat.InstanceID] = ctn
	}

	for _, ctn := range ctnMap {
		ctns = append(ctns, ctn)
	}

	return ctns, err
}

func (t *MixinService) MixinPod(pods []model.Pod) (p []model.Pod, err error) {
	podMap := make(map[int64]model.Pod)
	podIDs := make([]int64, 0)
	for _, pod := range pods {
		podMap[pod.ID] = pod
		podIDs = append(podIDs, pod.ID)
	}

	// mixin instance -> pod
	containers, err := t.ContainerRepository.FindByPodID(podIDs)
	containers, err = t.MixinInstance(containers)
	for _, container := range containers {
		pod := podMap[container.PodID]
		pod.Instances = append(pod.Instances, container)
		podMap[container.PodID] = pod
	}

	for _, pod := range podMap {
		p = append(p, pod)
	}

	return p, err
}
