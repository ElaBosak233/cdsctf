package service

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/container/manager"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/elabosak233/cloudsdale/pkg/convertor"
	"github.com/elabosak233/cloudsdale/pkg/generator"
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
	PodManagers = make(map[uint]interface{})
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
	Create(req request.PodCreateRequest) (res response.PodStatusResponse, err error)
	Status(id uint) (rep response.PodStatusResponse, err error)
	Renew(req request.PodRenewRequest) (removedAt int64, err error)
	Remove(req request.PodRemoveRequest) (err error)
	FindById(id uint) (rep response.PodResponse, err error)
	Find(req request.PodFindRequest) (rep []response.PodResponse, err error)
}

type PodService struct {
	ChallengeRepository repository.IChallengeRepository
	PodRepository       repository.IPodRepository
	NatRepository       repository.INatRepository
	FlagGenRepository   repository.IFlagGenRepository
	InstanceRepository  repository.IInstanceRepository
}

func NewPodService(appRepository *repository.Repository) IPodService {
	return &PodService{
		ChallengeRepository: appRepository.ChallengeRepository,
		InstanceRepository:  appRepository.ContainerRepository,
		FlagGenRepository:   appRepository.FlagGenRepository,
		PodRepository:       appRepository.PodRepository,
		NatRepository:       appRepository.NatRepository,
	}
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

func (t *PodService) Create(req request.PodCreateRequest) (res response.PodStatusResponse, err error) {
	remainder := t.IsLimited(req.UserID, int64(config.AppCfg().Global.Container.RequestLimit))
	if remainder != 0 {
		return res, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	switch config.AppCfg().Container.Provider {
	case "docker":
		SetUserInstanceRequestMap(req.UserID, time.Now().Unix()) // 保存用户请求时间
		challenges, _, _ := t.ChallengeRepository.Find(request.ChallengeFindRequest{
			IDs:       []uint{req.ChallengeID},
			IsDynamic: convertor.TrueP(),
		})
		challenge := challenges[0]
		isGame := req.GameID != nil && req.TeamID != nil

		// Parallel container limit
		if config.AppCfg().Global.Container.ParallelLimit > 0 {
			var availablePods []model.Pod
			var count int64
			if !isGame {
				availablePods, count, _ = t.PodRepository.Find(request.PodFindRequest{
					UserID:      req.UserID,
					IsAvailable: convertor.TrueP(),
				})
			} else {
				availablePods, count, _ = t.PodRepository.Find(request.PodFindRequest{
					TeamID:      req.TeamID,
					GameID:      req.GameID,
					IsAvailable: convertor.TrueP(),
				})
			}
			needToBeDeactivated := count - int64(config.AppCfg().Global.Container.ParallelLimit) + 1
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

		removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix()

		// Select the first one as the target flag which will be injected
		var flag model.Flag
		for _, f := range challenge.Flags {
			if f.Type == "dynamic" {
				flag = *f
				flag.Value = generator.GenerateFlag(flag.Value)
			} else if f.Type == "static" {
				flag = *f
				flag.Value = f.Value
			}
		}

		ctnManager := manager.NewDockerManager(
			challenge.Images,
			flag,
			time.Duration(challenge.Duration)*time.Second,
		)

		instances, err := ctnManager.Setup()

		// Insert Pod model, get Pod's ID
		pod, _ := t.PodRepository.Insert(model.Pod{
			ChallengeID: req.ChallengeID,
			UserID:      req.UserID,
			RemovedAt:   removedAt,
			Instances:   instances,
		})

		ctnManager.SetPodID(pod.ID)

		_, _ = t.FlagGenRepository.Insert(model.FlagGen{
			Flag:  flag.Value,
			PodID: pod.ID,
		})

		go func() {
			if ctnManager.RemoveAfterDuration(ctnManager.CancelCtx) {
				delete(PodManagers, pod.ID)
			}
		}()

		PodManagers[pod.ID] = ctnManager

		return response.PodStatusResponse{
			ID:        pod.ID,
			Instances: pod.Instances,
			RemovedAt: removedAt,
		}, err
	case "k8s":
		//SetUserInstanceRequestMap(req.UserID, time.Now().Unix()) // 保存用户请求时间
		//challenges, _, _ := t.ChallengeRepository.Find(request.ChallengeFindRequest{
		//	IDs:       []uint{req.ChallengeID},
		//	IsDynamic: convertor.TrueP(),
		//})
		//challenges, _ = t.MixinService.MixinChallenge(challenges)
		//challenge := challenges[0]
		//isGame := req.GameID != nil && req.TeamID != nil
		//
		//// Parallel container limit
		//if config.AppCfg().Global.Container.ParallelLimit > 0 {
		//	var availablePods []model.Pod
		//	var count int64
		//	if !isGame {
		//		availablePods, count, _ = t.PodRepository.Find(request.PodFindRequest{
		//			UserID:      req.UserID,
		//			IsAvailable: convertor.TrueP(),
		//		})
		//	} else {
		//		availablePods, count, _ = t.PodRepository.Find(request.PodFindRequest{
		//			TeamID:      req.TeamID,
		//			GameID:      req.GameID,
		//			IsAvailable: convertor.TrueP(),
		//		})
		//	}
		//	needToBeDeactivated := count - int64(config.AppCfg().Global.Container.ParallelLimit) + 1
		//	if needToBeDeactivated > 0 {
		//		for _, pod := range availablePods {
		//			if needToBeDeactivated == 0 {
		//				break
		//			}
		//			go func() {
		//				_ = t.Remove(request.PodRemoveRequest{
		//					ID: pod.ID,
		//				})
		//			}()
		//			needToBeDeactivated -= 1
		//		}
		//	}
		//}
		//
		//var ctnMap = make(map[int64]model.Instance)
		//
		//removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix()
		//
		//// Insert Pod model, get Pod's ID
		//pod, _ := t.PodRepository.Insert(model.Pod{
		//	ChallengeID: req.ChallengeID,
		//	UserID:      req.UserID,
		//	RemovedAt:   removedAt,
		//})
		//
		//// Select the first one as the target flag which will be injected
		//var flag model.Flag
		//var flagStr string
		//for _, f := range challenge.Flags {
		//	if f.Type == "dynamic" {
		//		flag = *f
		//		flagStr = generator.GenerateFlag(flag.Value)
		//	} else if f.Type == "static" {
		//		flag = *f
		//		flagStr = f.Value
		//	}
		//}
		//
		//_, _ = t.FlagGenRepository.Insert(model.FlagGen{
		//	Flag:  flagStr,
		//	PodID: pod.ID,
		//})
		//
		//ctnManager := manager.NewK8sManager(
		//	pod.ID,
		//	challenge.Images,
		//	flag,
		//	time.Duration(challenge.Duration),
		//)
		//
		//instances, _ := ctnManager.Setup()
		//
		//for _, instance := range instances {
		//	for _, nat := range instance.Nats {
		//		nat.InstanceID = instance.ID
		//		_, _ = t.NatRepository.Insert(*nat)
		//	}
		//	_, _ = t.InstanceRepository.Insert(*instance)
		//}
		//go func() {
		//	if ctnManager.RemoveAfterDuration(ctnManager.CancelCtx) {
		//		delete(PodManagers, pod.ID)
		//	}
		//}()
		//var ctns []model.Instance
		//for _, ctn := range ctnMap {
		//	ctns = append(ctns, ctn)
		//}
		//return response.PodStatusResponse{
		//	ID:        pod.ID,
		//	Instances: instances,
		//	RemovedAt: removedAt,
		//}, err
	}
	return res, errors.New("创建失败")
}

func (t *PodService) Status(podID uint) (rep response.PodStatusResponse, err error) {
	rep = response.PodStatusResponse{}
	if config.AppCfg().Container.Provider == "docker" {
		instance, err := t.PodRepository.FindById(podID)
		if PodManagers[podID] != nil {
			ctn := PodManagers[podID].(*manager.DockerManager)
			status, _ := ctn.GetContainerStatus()
			if status != "removed" {
				rep.ID = podID
				rep.Status = status
				rep.RemovedAt = instance.RemovedAt
				return rep, nil
			}
		}
		rep.Status = "removed"
		return rep, err
	}
	return rep, errors.New("获取失败")
}

func (t *PodService) Renew(req request.PodRenewRequest) (removedAt int64, err error) {
	remainder := t.IsLimited(req.UserID, int64(config.AppCfg().Global.Container.RequestLimit))
	if remainder != 0 {
		return 0, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if config.AppCfg().Container.Provider == "docker" {
		SetUserInstanceRequestMap(req.UserID, time.Now().Unix()) // 保存用户请求时间
		pod, _ := t.PodRepository.FindById(req.ID)
		ctn, ok := PodManagers[req.ID]
		if !ok {
			return 0, errors.New("实例不存在")
		}
		duration := ctn.(*manager.DockerManager).Duration
		ctn.(*manager.DockerManager).Renew(duration)
		pod.RemovedAt = time.Now().Add(duration).Unix()
		err = t.PodRepository.Update(pod)
		return pod.RemovedAt, err
	}
	return 0, errors.New("续期失败")
}

func (t *PodService) Remove(req request.PodRemoveRequest) (err error) {
	remainder := t.IsLimited(req.UserID, int64(config.AppCfg().Global.Container.RequestLimit))
	if remainder != 0 {
		return errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if config.AppCfg().Container.Provider == "docker" {
		_ = t.PodRepository.Update(model.Pod{
			ID:        req.ID,
			RemovedAt: time.Now().Unix(),
		})
		if ctn, ok := PodManagers[req.ID]; ok {
			ctn.(*manager.DockerManager).Remove()
		}
		return err
	}
	go func() {
		delete(PodManagers, req.ID)
	}()
	return errors.New("移除失败")
}

func (t *PodService) FindById(id uint) (rep response.PodResponse, err error) {
	if config.AppCfg().Container.Provider == "docker" {
		instance, _ := t.PodRepository.FindById(id)
		if PodManagers[id] != nil {
			ctn := PodManagers[id].(*manager.DockerManager)
			status, _ := ctn.GetContainerStatus()
			rep = response.PodResponse{
				ID:          id,
				RemovedAt:   instance.RemovedAt,
				ChallengeID: instance.ChallengeID,
				Status:      status,
			}
			return rep, nil
		}
	}
	return rep, errors.New("获取失败")
}

func (t *PodService) Find(req request.PodFindRequest) (pods []response.PodResponse, err error) {
	if config.AppCfg().Container.Provider == "docker" {
		if req.TeamID != nil && req.GameID != nil {
			req.UserID = 0
		}
		podResponse, _, err := t.PodRepository.Find(req)
		for _, pod := range podResponse {
			status := "removed"
			if PodManagers[pod.ID] != nil {
				ctn := PodManagers[pod.ID].(*manager.DockerManager)
				s, _ := ctn.GetContainerStatus()
				if s == "running" {
					status = s
				}
			}
			pods = append(pods, response.PodResponse{
				ID:          pod.ID,
				RemovedAt:   pod.RemovedAt,
				Instances:   pod.Instances,
				ChallengeID: pod.ChallengeID,
				Status:      status,
			})
		}
		return pods, err
	}
	return nil, errors.New("获取失败")
}
