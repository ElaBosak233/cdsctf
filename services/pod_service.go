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
	"github.com/elabosak233/pgshub/utils/convertor"
	"github.com/spf13/viper"
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

	// ContainerManagerPtrMap is a mapping of ContainerID and manager pointer.
	ContainerManagerPtrMap = make(map[any]interface{})

	// PodMap is a mapping of PodID and ContainerID
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
	Create(req request.InstanceCreateRequest) (res response.PodStatusResponse, err error)
	Status(id int64) (rep response.PodStatusResponse, err error)
	Renew(req request.InstanceRenewRequest) (removedAt int64, err error)
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
		podMap[pod.PodID] = pod
		podIDs = append(podIDs, pod.PodID)
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

func (t *PodServiceImpl) Create(req request.InstanceCreateRequest) (res response.PodStatusResponse, err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container.request_limit"))
	if remainder != 0 {
		return res, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("container.provider") == "docker" {
		SetUserInstanceRequestMap(req.UserId, time.Now().Unix()) // 保存用户请求时间
		challenges, _, _, _ := t.ChallengeService.Find(request.ChallengeFindRequest{
			ChallengeIds: []int64{req.ChallengeId},
			IsDynamic:    convertor.TrueP(),
		})
		challenge := challenges[0]
		isAvailable := true
		availableInstances, count, err := t.PodRepository.Find(request.PodFindRequest{
			UserId:      req.UserId,
			TeamId:      req.TeamId,
			GameId:      req.GameId,
			IsAvailable: &isAvailable,
		})
		if req.TeamId == 0 && req.GameId == 0 { // 练习场限制并行
			needToBeDeactivated := count - viper.GetInt64("global.container.parallel_limit")
			if needToBeDeactivated > 0 {
				for _, instance := range availableInstances {
					if needToBeDeactivated == 0 {
						break
					}
					go func() {
						err = t.Remove(request.PodRemoveRequest{
							PodID: instance.PodID,
						})
						if err != nil {
							fmt.Println(err)
						}
					}()
					needToBeDeactivated -= 1
				}
			}
		} else if req.TeamId != 0 && req.GameId != 0 { // 比赛限制并行
			// TODO
		}

		var ctnMap = make(map[int64]entity.Container)

		removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix()

		pod, _ := t.PodRepository.Insert(entity.Pod{
			ChallengeID: req.ChallengeId,
			UserID:      req.UserId,
			RemovedAt:   removedAt,
		})

		if _, ok := PodMap[pod.PodID]; !ok {
			PodMap[pod.PodID] = make([]int64, 0)
		}

		for _, image := range challenge.Images {
			var envs = make([]entity.Env, 0)
			for _, e := range image.Envs {
				envs = append(envs, e)
			}
			var flag entity.Flag
			for _, f := range challenge.Flags {
				if f.Type == "dynamic" {
					flag = f
					break
				}
			}
			flagStr := utils.GenerateFlag(flag.Content)

			// This Env is only a temporary entity. It will not be persisted.
			envs = append(envs, entity.Env{
				Key:   flag.Env,
				Value: flagStr,
			})

			_, _ = t.FlagGenRepository.Insert(entity.FlagGen{
				Flag:  flagStr,
				PodID: pod.PodID,
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
				ChallengeID: req.ChallengeId,
				PodID:       pod.PodID,
				ImageID:     image.ImageID,
			})
			PodMap[pod.PodID] = append(PodMap[pod.PodID], container.ContainerID)

			ctn := ctnMap[container.ContainerID]
			for src, dsts := range assignedPorts {
				srcPort := convertor.ToIntD(strings.Split(string(src), "/")[0], 0)
				for _, dst := range dsts {
					dstPort := convertor.ToIntD(dst.HostPort, 0)
					entry := fmt.Sprintf(
						"%s:%d",
						viper.GetString("container.docker.public_entry"),
						dstPort,
					)
					// Nat -> Container
					nat, _ := t.NatRepository.Insert(entity.Nat{
						ContainerID: container.ContainerID,
						SrcPort:     srcPort,
						DstPort:     dstPort,
						Entry:       entry,
					})
					ctn.Nats = append(ctn.Nats, nat)
				}
			}
			ctnMap[container.ContainerID] = ctn

			ctnManager.SetContainerID(container.ContainerID)
			ContainerManagerPtrMap[container.ContainerID] = ctnManager

			// Start removal plan
			go func() {
				if ctnManager.RemoveAfterDuration(ctnManager.CancelCtx) {
					delete(ContainerManagerPtrMap, container.ContainerID)
				}
			}()
		}
		var ctns []entity.Container
		for _, ctn := range ctnMap {
			ctns = append(ctns, ctn)
		}
		return response.PodStatusResponse{
			PodID:      pod.PodID,
			Containers: ctns,
			RemovedAt:  removedAt,
		}, err
	}
	return res, errors.New("创建失败")
}

func (t *PodServiceImpl) Status(podID int64) (rep response.PodStatusResponse, err error) {
	rep = response.PodStatusResponse{}
	if viper.GetString("container.provider") == "docker" {
		instance, err := t.PodRepository.FindById(podID)
		if ContainerManagerPtrMap[podID] != nil {
			ctn := ContainerManagerPtrMap[podID].(*managers.DockerManager)
			status, _ := ctn.GetContainerStatus()
			if status != "removed" {
				rep.PodID = podID
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

func (t *PodServiceImpl) Renew(req request.InstanceRenewRequest) (removedAt int64, err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container.request_limit"))
	if remainder != 0 {
		return 0, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("Container.Provider") == "docker" {
		SetUserInstanceRequestMap(req.UserId, time.Now().Unix()) // 保存用户请求时间
		instance, err := t.PodRepository.FindById(req.InstanceId)
		if err != nil || ContainerManagerPtrMap[req.InstanceId] == nil {
			return 0, errors.New("实例不存在")
		}
		ctn := ContainerManagerPtrMap[req.InstanceId].(*managers.DockerManager)
		ctn.Renew(ctn.Duration)
		instance.RemovedAt = time.Now().Add(ctn.Duration).Unix()
		err = t.PodRepository.Update(instance)
		return instance.RemovedAt, err
	}
	return 0, errors.New("续期失败")
}

func (t *PodServiceImpl) Remove(req request.PodRemoveRequest) (err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container.request_limit"))
	if remainder != 0 {
		return errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("container.provider") == "docker" {
		_ = t.PodRepository.Update(entity.Pod{
			PodID:     req.PodID,
			RemovedAt: time.Now().Unix(),
		})
		if ContainerManagerPtrMap[req.PodID] != nil {
			ctn := ContainerManagerPtrMap[req.PodID].(*managers.DockerManager)
			ctn.Remove()
		}
		return err
	}
	go func() {
		delete(ContainerManagerPtrMap, req.PodID)
	}()
	return errors.New("移除失败")
}

func (t *PodServiceImpl) FindById(id int64) (rep response.PodResponse, err error) {
	if viper.GetString("container.provider") == "docker" {
		instance, err := t.PodRepository.FindById(id)
		if err != nil || ContainerManagerPtrMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		ctn := ContainerManagerPtrMap[id].(*managers.DockerManager)
		status, _ := ctn.GetContainerStatus()
		rep = response.PodResponse{
			PodID:       id,
			RemovedAt:   instance.RemovedAt,
			ChallengeID: instance.ChallengeID,
			Status:      status,
		}
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *PodServiceImpl) Find(req request.PodFindRequest) (pods []response.PodResponse, err error) {
	if viper.GetString("container.provider") == "docker" {
		if req.TeamId != 0 && req.GameId != 0 {
			req.UserId = 0
		}
		podResponse, _, err := t.PodRepository.Find(req)
		podResponse, err = t.Mixin(podResponse)
		for _, pod := range podResponse {
			var ctn *managers.DockerManager
			status := "removed"
			if ContainerManagerPtrMap[pod.PodID] != nil {
				ctn = ContainerManagerPtrMap[pod.PodID].(*managers.DockerManager)
				status, _ = ctn.GetContainerStatus()
			}
			pods = append(pods, response.PodResponse{
				PodID:       pod.PodID,
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
