package services

import (
	"errors"
	"fmt"
	"github.com/elabosak233/pgshub/containers/managers"
	model "github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/utils"
	"github.com/spf13/viper"
	"sync"
	"time"
)

var (
	// UserInstanceRequestMap 用于存储用户上次请求的时间
	UserInstanceRequestMap = struct {
		sync.RWMutex
		m map[int64]int64
	}{m: make(map[int64]int64)}

	// InstanceMap 存储当前状态下所有的实例
	InstanceMap = make(map[any]interface{})
)

// GetUserInstanceRequestMap 返回用户上次请求的时间
func GetUserInstanceRequestMap(userId int64) int64 {
	UserInstanceRequestMap.RLock()
	defer UserInstanceRequestMap.RUnlock()
	return UserInstanceRequestMap.m[userId]
}

// SetUserInstanceRequestMap 设置用户上次请求的时间
func SetUserInstanceRequestMap(userId int64, t int64) {
	UserInstanceRequestMap.Lock()
	defer UserInstanceRequestMap.Unlock()
	UserInstanceRequestMap.m[userId] = t
}

type InstanceService interface {
	Create(req request.InstanceCreateRequest) (res response.InstanceStatusResponse, err error)
	Status(id int64) (rep response.InstanceStatusResponse, err error)
	Renew(req request.InstanceRenewRequest) (removedAt time.Time, err error)
	Remove(req request.InstanceRemoveRequest) (err error)
	FindById(id int64) (rep response.InstanceResponse, err error)
	Find(req request.InstanceFindRequest) (rep []response.InstanceResponse, err error)
}

type InstanceServiceImpl struct {
	ChallengeRepository repositories.ChallengeRepository
	InstanceRepository  repositories.InstanceRepository
}

func NewInstanceServiceImpl(appRepository *repositories.Repositories) InstanceService {
	return &InstanceServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
		InstanceRepository:  appRepository.InstanceRepository,
	}
}

func (t *InstanceServiceImpl) IsLimited(userId int64, limit int64) (remainder int64) {
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

func (t *InstanceServiceImpl) Create(req request.InstanceCreateRequest) (res response.InstanceStatusResponse, err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container.request_limit"))
	if remainder != 0 {
		return res, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("container.provider") == "docker" {
		SetUserInstanceRequestMap(req.UserId, time.Now().Unix()) // 保存用户请求时间
		challenge, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
		availableInstances, count, err := t.InstanceRepository.Find(request.InstanceFindRequest{
			UserId:      req.UserId,
			TeamId:      req.TeamId,
			GameId:      req.GameId,
			IsAvailable: 1,
		})
		if req.TeamId == 0 && req.GameId == 0 { // 练习场限制并行
			needToBeDeactivated := count - viper.GetInt64("global.container.parallel_limit")
			if needToBeDeactivated > 0 {
				for _, instance := range availableInstances {
					if needToBeDeactivated == 0 {
						break
					}
					go func() {
						err = t.Remove(request.InstanceRemoveRequest{
							InstanceId: instance.InstanceId,
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
		flag := utils.GenerateFlag(challenge.FlagFmt)
		ctn := managers.NewDockerManagerImpl(
			challenge.Image,
			challenge.ExposedPort,
			flag,
			challenge.FlagEnv,
			challenge.MemoryLimit,
			challenge.CpuLimit,
			time.Duration(challenge.Duration)*time.Second)
		port, err := ctn.Setup()
		entry := fmt.Sprintf("%s:%d", viper.GetString("container.docker.public_entry"), port)
		removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second)
		instance, err := t.InstanceRepository.Insert(model.Instance{
			ChallengeId: req.ChallengeId,
			UserId:      req.UserId,
			Flag:        flag,
			Entry:       entry,
			RemovedAt:   removedAt,
		})
		ctn.SetInstanceId(instance.InstanceId)
		InstanceMap[instance.InstanceId] = ctn
		go func() {
			if ctn.RemoveAfterDuration(ctn.CancelCtx) {
				delete(InstanceMap, instance.InstanceId)
			}
		}()
		return response.InstanceStatusResponse{
			InstanceId: instance.InstanceId,
			Entry:      entry,
			RemovedAt:  removedAt,
			Status:     "running",
		}, err
	}
	return res, errors.New("创建失败")
}

func (t *InstanceServiceImpl) Status(id int64) (rep response.InstanceStatusResponse, err error) {
	rep = response.InstanceStatusResponse{}
	if viper.GetString("container.provider") == "docker" {
		instance, err := t.InstanceRepository.FindById(id)
		if InstanceMap[id] != nil {
			ctn := InstanceMap[id].(*managers.DockerManager)
			status, _ := ctn.GetContainerStatus()
			if status != "removed" {
				rep.InstanceId = id
				rep.Status = status
				rep.Entry = instance.Entry
				rep.RemovedAt = instance.RemovedAt
				return rep, nil
			}
		}
		rep.Status = "removed"
		return rep, err
	}
	return rep, errors.New("获取失败")
}

func (t *InstanceServiceImpl) Renew(req request.InstanceRenewRequest) (removedAt time.Time, err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container.request_limit"))
	if remainder != 0 {
		return time.Time{}, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("Container.Provider") == "docker" {
		SetUserInstanceRequestMap(req.UserId, time.Now().Unix()) // 保存用户请求时间
		instance, err := t.InstanceRepository.FindById(req.InstanceId)
		if err != nil || InstanceMap[req.InstanceId] == nil {
			return time.Time{}, errors.New("实例不存在")
		}
		ctn := InstanceMap[req.InstanceId].(*managers.DockerManager)
		err = ctn.Renew(ctn.Duration)
		instance.RemovedAt = time.Now().Add(ctn.Duration)
		err = t.InstanceRepository.Update(instance)
		return instance.RemovedAt, err
	}
	return time.Time{}, errors.New("续期失败")
}

func (t *InstanceServiceImpl) Remove(req request.InstanceRemoveRequest) (err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container.request_limit"))
	if remainder != 0 {
		return errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("container.provider") == "docker" {
		_ = t.InstanceRepository.Update(model.Instance{
			InstanceId: req.InstanceId,
			RemovedAt:  time.Now(),
		})
		if InstanceMap[req.InstanceId] != nil {
			ctn := InstanceMap[req.InstanceId].(*managers.DockerManager)
			err = ctn.Remove()
		}
		return err
	}
	go func() {
		delete(InstanceMap, req.InstanceId)
	}()
	return errors.New("移除失败")
}

func (t *InstanceServiceImpl) FindById(id int64) (rep response.InstanceResponse, err error) {
	if viper.GetString("container.provider") == "docker" {
		instance, err := t.InstanceRepository.FindById(id)
		if err != nil || InstanceMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		ctn := InstanceMap[id].(*managers.DockerManager)
		status, _ := ctn.GetContainerStatus()
		rep = response.InstanceResponse{
			InstanceId:  id,
			Entry:       instance.Entry,
			RemovedAt:   instance.RemovedAt,
			ChallengeId: instance.ChallengeId,
			Status:      status,
		}
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *InstanceServiceImpl) Find(req request.InstanceFindRequest) (instances []response.InstanceResponse, err error) {
	if viper.GetString("container.provider") == "docker" {
		if req.TeamId != 0 && req.GameId != 0 {
			req.UserId = 0
		}
		responses, _, err := t.InstanceRepository.Find(req)
		for _, instance := range responses {
			var ctn *managers.DockerManager
			status := "removed"
			if InstanceMap[instance.InstanceId] != nil {
				ctn = InstanceMap[instance.InstanceId].(*managers.DockerManager)
				status, _ = ctn.GetContainerStatus()
			}
			instances = append(instances, response.InstanceResponse{
				InstanceId:  instance.InstanceId,
				Entry:       instance.Entry,
				RemovedAt:   instance.RemovedAt,
				ChallengeId: instance.ChallengeId,
				Status:      status,
			})
		}
		return instances, err
	}
	return nil, errors.New("获取失败")
}
