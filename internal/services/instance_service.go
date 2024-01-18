package services

import (
	"errors"
	"fmt"
	"github.com/elabosak233/pgshub/internal"
	"github.com/elabosak233/pgshub/internal/containers/managers"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/spf13/viper"
	"time"
)

type InstanceService interface {
	Create(req request.InstanceCreateRequest) (res response.InstanceStatusResponse, err error)
	Status(id int64) (rep response.InstanceStatusResponse, err error)
	Renew(req request.InstanceRenewRequest) (removedAt int64, err error)
	Remove(req request.InstanceRemoveRequest) (err error)
	FindById(id int64) (rep response.InstanceResponse, err error)
	Find(req request.InstanceFindRequest) (rep []response.InstanceResponse, err error)
}

type InstanceServiceImpl struct {
	ChallengeRepository repositories.ChallengeRepository
	InstanceRepository  repositories.InstanceRepository
}

func NewInstanceServiceImpl(appRepository *repositories.AppRepository) InstanceService {
	return &InstanceServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
		InstanceRepository:  appRepository.InstanceRepository,
	}
}

func (t *InstanceServiceImpl) IsLimited(userId int64, limit int64) (remainder int64) {
	if userId == 0 {
		return 0
	}
	ti := internal.GetUserInstanceRequestMap(userId)
	if ti != 0 {
		if time.Now().Unix()-ti < limit {
			return limit - (time.Now().Unix() - ti)
		}
	}
	return 0
}

func (t *InstanceServiceImpl) Create(req request.InstanceCreateRequest) (res response.InstanceStatusResponse, err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container_request_limit"))
	if remainder != 0 {
		return res, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("container.provider") == "docker" {
		internal.SetUserInstanceRequestMap(req.UserId, time.Now().Unix()) // 保存用户请求时间
		challenge, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
		availableInstances, count, err := t.InstanceRepository.Find(request.InstanceFindRequest{
			UserId:      req.UserId,
			TeamId:      req.TeamId,
			GameId:      req.GameId,
			IsAvailable: 1,
		})
		if req.TeamId == 0 && req.GameId == 0 { // 练习场限制并行
			needToBeDeactivated := count - viper.GetInt64("global.parallel_container_limit")
			fmt.Println(count, needToBeDeactivated)
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
		removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix()
		instance, err := t.InstanceRepository.Insert(model.Instance{
			ChallengeId: req.ChallengeId,
			UserId:      req.UserId,
			Flag:        flag,
			Entry:       entry,
			RemovedAt:   removedAt,
		})
		ctn.SetInstanceId(instance.InstanceId)
		internal.InstanceMap[instance.InstanceId] = ctn
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
		if internal.InstanceMap[id] != nil {
			ctn := internal.InstanceMap[id].(*managers.DockerManager)
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

func (t *InstanceServiceImpl) Renew(req request.InstanceRenewRequest) (removedAt int64, err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container_request_limit"))
	if remainder != 0 {
		return 0, errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("Container.Provider") == "docker" {
		internal.SetUserInstanceRequestMap(req.UserId, time.Now().Unix()) // 保存用户请求时间
		instance, err := t.InstanceRepository.FindById(req.InstanceId)
		if err != nil || internal.InstanceMap[req.InstanceId] == nil {
			return 0, errors.New("实例不存在")
		}
		ctn := internal.InstanceMap[req.InstanceId].(*managers.DockerManager)
		err = ctn.Renew(ctn.Duration)
		instance.RemovedAt = time.Now().Add(ctn.Duration).Unix()
		err = t.InstanceRepository.Update(instance)
		return instance.RemovedAt, err
	}
	return 0, errors.New("续期失败")
}

func (t *InstanceServiceImpl) Remove(req request.InstanceRemoveRequest) (err error) {
	remainder := t.IsLimited(req.UserId, viper.GetInt64("global.container_request_limit"))
	if remainder != 0 {
		return errors.New(fmt.Sprintf("请等待 %d 秒后再次请求", remainder))
	}
	if viper.GetString("container.provider") == "docker" {
		_ = t.InstanceRepository.Update(model.Instance{
			InstanceId: req.InstanceId,
			RemovedAt:  time.Now().Unix(),
		})
		if internal.InstanceMap[req.InstanceId] != nil {
			ctn := internal.InstanceMap[req.InstanceId].(*managers.DockerManager)
			err = ctn.Remove()
		}
		return err
	}
	return errors.New("移除失败")
}

func (t *InstanceServiceImpl) FindById(id int64) (rep response.InstanceResponse, err error) {
	if viper.GetString("container.provider") == "docker" {
		instance, err := t.InstanceRepository.FindById(id)
		if err != nil || internal.InstanceMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		ctn := internal.InstanceMap[id].(*managers.DockerManager)
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
			if internal.InstanceMap[instance.InstanceId] != nil {
				ctn = internal.InstanceMap[instance.InstanceId].(*managers.DockerManager)
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
