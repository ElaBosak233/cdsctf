package service

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"github.com/elabosak233/cloudsdale/internal/extension/container/manager"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"strings"
	"sync"
	"time"
)

var (
	// UserPodRequestMap 用于存储用户上次请求的时间
	UserPodRequestMap = struct {
		sync.RWMutex
		m map[uint]int64
	}{m: make(map[uint]int64)}

	// PodManagers is a mapping of PodID and manager pointer.
	PodManagers = make(map[uint]manager.IContainerManager)
)

// GetUserInstanceRequestMap 返回用户上次请求的时间
func GetUserInstanceRequestMap(userID uint) int64 {
	UserPodRequestMap.RLock()
	defer UserPodRequestMap.RUnlock()
	return UserPodRequestMap.m[userID]
}

// SetUserInstanceRequestMap 设置用户上次请求的时间
func SetUserInstanceRequestMap(userID uint, t int64) {
	UserPodRequestMap.Lock()
	defer UserPodRequestMap.Unlock()
	UserPodRequestMap.m[userID] = t
}

type IPodService interface {
	Create(req request.PodCreateRequest) (model.Pod, error)
	Renew(req request.PodRenewRequest) error
	Remove(req request.PodRemoveRequest) error
	Find(req request.PodFindRequest) ([]model.Pod, int64, error)
}

type PodService struct {
	challengeRepository repository.IChallengeRepository
	podRepository       repository.IPodRepository
	natRepository       repository.INatRepository
	flagGenRepository   repository.IFlagGenRepository
}

func NewPodService(appRepository *repository.Repository) IPodService {
	return &PodService{
		challengeRepository: appRepository.ChallengeRepository,
		flagGenRepository:   appRepository.FlagGenRepository,
		podRepository:       appRepository.PodRepository,
		natRepository:       appRepository.NatRepository,
	}
}

func GenerateFlag(flagFmt string) (flag string) {
	flag = strings.Replace(flagFmt, "[UUID]", utils.HyphenlessUUID(), -1)
	return flag
}

func (t *PodService) IsLimited(userID uint, limit int64) (remainder int64) {
	if userID == 0 {
		return 0
	}
	ti := GetUserInstanceRequestMap(userID)
	if ti != 0 {
		if time.Now().Unix()-ti < limit {
			return limit - (time.Now().Unix() - ti)
		}
	}
	return 0
}

func (t *PodService) ParallelLimit(req request.PodCreateRequest) {
	isGame := req.GameID != nil && req.TeamID != nil
	if config.PltCfg().Container.ParallelLimit > 0 {
		var availablePods []model.Pod
		var count int64
		if !isGame {
			availablePods, count, _ = t.podRepository.Find(request.PodFindRequest{
				UserID:      &req.UserID,
				IsAvailable: convertor.TrueP(),
			})
		} else {
			availablePods, count, _ = t.podRepository.Find(request.PodFindRequest{
				TeamID:      req.TeamID,
				GameID:      req.GameID,
				IsAvailable: convertor.TrueP(),
			})
		}
		needToBeDeactivated := count - int64(config.PltCfg().Container.ParallelLimit) + 1
		if needToBeDeactivated > 0 {
			for _, pod := range availablePods {
				if needToBeDeactivated == 0 {
					break
				}
				go func() {
					_ = t.Remove(request.PodRemoveRequest{
						ID: pod.ID,
					})
				}()
				needToBeDeactivated -= 1
			}
		}
	}
}

func (t *PodService) Create(req request.PodCreateRequest) (model.Pod, error) {
	remainder := t.IsLimited(req.UserID, int64(config.PltCfg().Container.RequestLimit))
	if remainder != 0 {
		return model.Pod{}, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	SetUserInstanceRequestMap(req.UserID, time.Now().Unix())
	challenges, _, _ := t.challengeRepository.Find(request.ChallengeFindRequest{
		ID:        req.ChallengeID,
		IsDynamic: convertor.TrueP(),
	})
	challenge := challenges[0]

	t.ParallelLimit(req)

	removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix()

	// Select the first one as the target flag which will be injected
	var flag model.Flag
	for _, f := range challenge.Flags {
		if f.Type == "dynamic" {
			flag = *f
			flag.Value = GenerateFlag(flag.Value)
		} else {
			if f.Env == "" {
				f.Env = "FLAG"
			}
			flag = *f
		}
		break
	}

	ctnManager := manager.NewContainerManager(
		challenge,
		flag,
		time.Duration(challenge.Duration)*time.Second,
	)

	nats, err := ctnManager.Setup()

	// Create Pod model, get Pod's GameID
	pod, _ := t.podRepository.Create(model.Pod{
		ChallengeID: &req.ChallengeID,
		UserID:      &req.UserID,
		GameID:      req.GameID,
		TeamID:      req.TeamID,
		RemovedAt:   removedAt,
		Nats:        nats,
	})

	ctnManager.SetPodID(pod.ID)

	_, _ = t.flagGenRepository.Create(model.FlagGen{
		Flag:  flag.Value,
		PodID: pod.ID,
	})

	go func() {
		if ctnManager.RemoveAfterDuration() {
			delete(PodManagers, pod.ID)
		}
	}()

	PodManagers[pod.ID] = ctnManager

	pod.Simplify()

	return pod, err
}

func (t *PodService) Renew(req request.PodRenewRequest) error {
	remainder := t.IsLimited(req.UserID, int64(config.PltCfg().Container.RequestLimit))
	if remainder != 0 {
		return errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	SetUserInstanceRequestMap(req.UserID, time.Now().Unix()) // 保存用户请求时间
	pod, _ := t.podRepository.FindById(req.ID)
	ctn, ok := PodManagers[req.ID]
	if !ok {
		return errors.New("实例不存在")
	}
	ctn.Renew(ctn.Duration())
	pod.RemovedAt = time.Now().Add(ctn.Duration()).Unix()
	err := t.podRepository.Update(pod)
	return err
}

func (t *PodService) Remove(req request.PodRemoveRequest) error {
	remainder := t.IsLimited(req.UserID, int64(config.PltCfg().Container.RequestLimit))
	if remainder != 0 {
		return errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	err := t.podRepository.Update(model.Pod{
		ID:        req.ID,
		RemovedAt: time.Now().Unix(),
	})
	if ctn, ok := PodManagers[req.ID]; ok {
		ctn.Remove()
	}
	go func() {
		delete(PodManagers, req.ID)
	}()
	return err
}

func (t *PodService) Find(req request.PodFindRequest) ([]model.Pod, int64, error) {
	if req.TeamID != nil && req.GameID != nil {
		req.UserID = nil
	}
	pods, total, err := t.podRepository.Find(req)

	for i, pod := range pods {
		pod.Simplify()
		pods[i] = pod
	}
	return pods, total, err
}
