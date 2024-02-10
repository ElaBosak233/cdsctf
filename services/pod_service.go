package services

import (
	"errors"
	"fmt"
	"github.com/elabosak233/pgshub/containers/managers"
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/utils"
	"github.com/elabosak233/pgshub/utils/config"
	"github.com/elabosak233/pgshub/utils/convertor"
	"strings"
	"sync"
	"time"
)

var (
	// UserPodRequestMap 用于存储用户上次请求的时间
	UserPodRequestMap = struct {
		sync.RWMutex
		m map[int64]int64
	}{m: make(map[int64]int64)}

	// ContainerManagerPtrMap is a mapping of ID and manager pointer.
	ContainerManagerPtrMap = make(map[any]interface{})

	// PodMap is a mapping of IDs and ID
	PodMap = make(map[int64][]int64)
)

// GetUserInstanceRequestMap 返回用户上次请求的时间
func GetUserInstanceRequestMap(userId int64) int64 {
	UserPodRequestMap.RLock()
	defer UserPodRequestMap.RUnlock()
	return UserPodRequestMap.m[userId]
}

// SetUserInstanceRequestMap 设置用户上次请求的时间
func SetUserInstanceRequestMap(userId int64, t int64) {
	UserPodRequestMap.Lock()
	defer UserPodRequestMap.Unlock()
	UserPodRequestMap.m[userId] = t
}

type PodService interface {
	Create(req request.PodCreateRequest) (res response.PodStatusResponse, err error)
	Status(id int64) (rep response.PodStatusResponse, err error)
	Renew(req request.PodRenewRequest) (removedAt int64, err error)
	Remove(req request.PodRemoveRequest) (err error)
	FindById(id int64) (rep response.PodResponse, err error)
	Find(req request.PodFindRequest) (rep []response.PodResponse, err error)
}

type PodServiceImpl struct {
	ChallengeService    ChallengeService
	ContainerService    ContainerService
	ChallengeRepository repositories.ChallengeRepository
	PodRepository       repositories.PodRepository
	NatRepository       repositories.NatRepository
	FlagGenRepository   repositories.FlagGenRepository
	ContainerRepository repositories.ContainerRepository
}

func NewPodServiceImpl(appRepository *repositories.Repositories) PodService {
	return &PodServiceImpl{
		ChallengeService:    NewChallengeServiceImpl(appRepository),
		ContainerService:    NewContainerServiceImpl(appRepository),
		ChallengeRepository: appRepository.ChallengeRepository,
		ContainerRepository: appRepository.ContainerRepository,
		FlagGenRepository:   appRepository.FlagGenRepository,
		PodRepository:       appRepository.PodRepository,
		NatRepository:       appRepository.NatRepository,
	}
}

func (t *PodServiceImpl) Mixin(pods []entity.Pod) (p []entity.Pod, err error) {
	podMap := make(map[int64]entity.Pod)
	podIDs := make([]int64, 0)
	for _, pod := range pods {
		podMap[pod.ID] = pod
		podIDs = append(podIDs, pod.ID)
	}

	// mixin container -> pod
	containers, err := t.ContainerService.FindByPodID(podIDs)
	for _, container := range containers {
		pod := podMap[container.PodID]
		pod.Containers = append(pod.Containers, container)
		podMap[container.PodID] = pod
	}

	for _, pod := range podMap {
		p = append(p, pod)
	}

	return p, err
}

func (t *PodServiceImpl) IsLimited(userId int64, limit int64) (remainder int64) {
	if userId == 0 {
		return 0
	}
	ti := GetUserInstanceRequestMap(userId)
	if ti != 0 {
		if time.Now().Unix()-ti < limit {
			return limit - (time.Now().Unix() - ti)
		}
	}
	return 0
}

func (t *PodServiceImpl) Create(req request.PodCreateRequest) (res response.PodStatusResponse, err error) {
	remainder := t.IsLimited(req.UserID, int64(config.Cfg().Global.Container.RequestLimit))
	if remainder != 0 {
		return res, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if config.Cfg().Container.Provider == "docker" {
		SetUserInstanceRequestMap(req.UserID, time.Now().Unix()) // 保存用户请求时间
		challenges, _, _, _ := t.ChallengeService.Find(request.ChallengeFindRequest{
			IDs:       []int64{req.ChallengeID},
			IsDynamic: convertor.TrueP(),
		})
		challenge := challenges[0]
		availableInstances, count, err := t.PodRepository.Find(request.PodFindRequest{
			UserID:      req.UserID,
			TeamID:      req.TeamID,
			GameID:      req.GameID,
			IsAvailable: convertor.TrueP(),
		})
		if req.TeamID == 0 && req.GameID == 0 { // 练习场限制并行
			needToBeDeactivated := count - int64(config.Cfg().Global.Container.ParallelLimit)
			if needToBeDeactivated > 0 {
				for _, instance := range availableInstances {
					if needToBeDeactivated == 0 {
						break
					}
					go func() {
						_ = t.Remove(request.PodRemoveRequest{
							ID: instance.ID,
						})
					}()
					needToBeDeactivated -= 1
				}
			}
		} else if req.TeamID != 0 && req.GameID != 0 { // 比赛限制并行
			// TODO
		}

		var ctnMap = make(map[int64]entity.Container)

		removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix()

		pod, _ := t.PodRepository.Insert(entity.Pod{
			ChallengeID: req.ChallengeID,
			UserID:      req.UserID,
			RemovedAt:   removedAt,
		})

		if _, ok := PodMap[pod.ID]; !ok {
			PodMap[pod.ID] = make([]int64, 0)
		}

		// Select the first one as the target flag which will be injected
		var flag entity.Flag
		var flagStr string
		for _, f := range challenge.Flags {
			if f.Type == "dynamic" {
				flag = f
				flagStr = utils.GenerateFlag(flag.Value)
			} else if f.Type == "static" {
				flag = f
				flagStr = f.Value
			}
		}

		_, _ = t.FlagGenRepository.Insert(entity.FlagGen{
			Flag:  flagStr,
			PodID: pod.ID,
		})

		for _, image := range challenge.Images {
			var envs = make([]entity.Env, 0)
			for _, e := range image.Envs {
				envs = append(envs, e)
			}

			// This Flag Env is only a temporary entity. It will not be persisted.
			envs = append(envs, entity.Env{
				Key:   flag.Env,
				Value: flagStr,
			})

			ctnManager := managers.NewDockerManagerImpl(
				image.Name,
				image.Ports,
				envs,
				image.MemoryLimit,
				image.CPULimit,
				time.Duration(challenge.Duration)*time.Second,
			)
			assignedPorts, _ := ctnManager.Setup()

			// Container -> Pod
			container, _ := t.ContainerRepository.Insert(entity.Container{
				PodID:   pod.ID,
				ImageID: image.ID,
			})
			PodMap[pod.ID] = append(PodMap[pod.ID], container.ID)

			ctn := ctnMap[container.ID]
			ctn.Image = nil
			for src, dsts := range assignedPorts {
				srcPort := convertor.ToIntD(strings.Split(string(src), "/")[0], 0)
				for _, dst := range dsts {
					dstPort := convertor.ToIntD(dst.HostPort, 0)
					entry := fmt.Sprintf(
						"%s:%d",
						config.Cfg().Container.Docker.PublicEntry,
						dstPort,
					)
					// Nat -> Container
					nat, _ := t.NatRepository.Insert(entity.Nat{
						ContainerID: container.ID,
						SrcPort:     srcPort,
						DstPort:     dstPort,
						Entry:       entry,
					})
					ctn.Nats = append(ctn.Nats, nat)
				}
			}
			ctnMap[container.ID] = ctn

			ctnManager.SetContainerID(container.ID)
			ContainerManagerPtrMap[container.ID] = ctnManager

			// Start removal plan
			go func() {
				if ctnManager.RemoveAfterDuration(ctnManager.CancelCtx) {
					delete(ContainerManagerPtrMap, container.ID)
				}
			}()
		}
		var ctns []entity.Container
		for _, ctn := range ctnMap {
			ctns = append(ctns, ctn)
		}
		return response.PodStatusResponse{
			ID:         pod.ID,
			Containers: ctns,
			RemovedAt:  removedAt,
		}, err
	}
	return res, errors.New("创建失败")
}

func (t *PodServiceImpl) Status(podID int64) (rep response.PodStatusResponse, err error) {
	rep = response.PodStatusResponse{}
	if config.Cfg().Container.Provider == "docker" {
		instance, err := t.PodRepository.FindById(podID)
		if ContainerManagerPtrMap[podID] != nil {
			ctn := ContainerManagerPtrMap[podID].(*managers.DockerManager)
			status, _ := ctn.GetContainerStatus()
			if status != "removed" {
				rep.ID = podID
				rep.Status = status
				//rep.Entry = instance.Entry
				rep.RemovedAt = instance.RemovedAt
				return rep, nil
			}
		}
		rep.Status = "removed"
		return rep, err
	}
	return rep, errors.New("获取失败")
}

func (t *PodServiceImpl) Renew(req request.PodRenewRequest) (removedAt int64, err error) {
	remainder := t.IsLimited(req.UserID, int64(config.Cfg().Global.Container.RequestLimit))
	if remainder != 0 {
		return 0, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if config.Cfg().Container.Provider == "docker" {
		SetUserInstanceRequestMap(req.UserID, time.Now().Unix()) // 保存用户请求时间
		pod, err := t.PodRepository.FindById(req.ID)
		if err != nil || PodMap[req.ID] == nil {
			return 0, errors.New("实例不存在")
		}
		var duration time.Duration
		for _, ctnID := range PodMap[req.ID] {
			if ContainerManagerPtrMap[ctnID] != nil {
				ctn := ContainerManagerPtrMap[ctnID].(*managers.DockerManager)
				duration = ctn.Duration
				ctn.Renew(duration)
			}
		}
		pod.RemovedAt = time.Now().Add(duration).Unix()
		err = t.PodRepository.Update(pod)
		return pod.RemovedAt, err
	}
	return 0, errors.New("续期失败")
}

func (t *PodServiceImpl) Remove(req request.PodRemoveRequest) (err error) {
	remainder := t.IsLimited(req.UserID, int64(config.Cfg().Global.Container.RequestLimit))
	if remainder != 0 {
		return errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if config.Cfg().Container.Provider == "docker" {
		_ = t.PodRepository.Update(entity.Pod{
			ID:        req.ID,
			RemovedAt: time.Now().Unix(),
		})
		for _, ctnID := range PodMap[req.ID] {
			if ContainerManagerPtrMap[ctnID] != nil {
				ctn := ContainerManagerPtrMap[ctnID].(*managers.DockerManager)
				ctn.Remove()
			}
		}
		return err
	}
	go func() {
		delete(ContainerManagerPtrMap, req.ID)
	}()
	return errors.New("移除失败")
}

func (t *PodServiceImpl) FindById(id int64) (rep response.PodResponse, err error) {
	if config.Cfg().Container.Provider == "docker" {
		instance, err := t.PodRepository.FindById(id)
		if err != nil || ContainerManagerPtrMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		ctn := ContainerManagerPtrMap[id].(*managers.DockerManager)
		status, _ := ctn.GetContainerStatus()
		rep = response.PodResponse{
			ID:          id,
			RemovedAt:   instance.RemovedAt,
			ChallengeID: instance.ChallengeID,
			Status:      status,
		}
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *PodServiceImpl) Find(req request.PodFindRequest) (pods []response.PodResponse, err error) {
	if config.Cfg().Container.Provider == "docker" {
		if req.TeamID != 0 && req.GameID != 0 {
			req.UserID = 0
		}
		podResponse, _, err := t.PodRepository.Find(req)
		podResponse, err = t.Mixin(podResponse)
		for _, pod := range podResponse {
			var ctn *managers.DockerManager
			status := "removed"
			for _, ctnID := range PodMap[pod.ID] {
				if ContainerManagerPtrMap[ctnID] != nil {
					ctn = ContainerManagerPtrMap[ctnID].(*managers.DockerManager)
					s, _ := ctn.GetContainerStatus()
					if s == "running" {
						status = s
					}
				}
			}
			pods = append(pods, response.PodResponse{
				ID:          pod.ID,
				RemovedAt:   pod.RemovedAt,
				Containers:  pod.Containers,
				ChallengeID: pod.ChallengeID,
				Status:      status,
			})
		}
		return pods, err
	}
	return nil, errors.New("获取失败")
}
