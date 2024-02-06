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
	"github.com/google/uuid"
	"github.com/spf13/viper"
	"time"
)

type InstanceService interface {
	Create(req request.InstanceCreateRequest) (res response.InstanceStatusResponse, err error)
	Status(id string) (rep response.InstanceStatusResponse, err error)
	Renew(id string) (removedAt int64, err error)
	Remove(id string) (err error)
	FindById(id string) (rep response.InstanceResponse, err error)
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

func (t *InstanceServiceImpl) Create(req request.InstanceCreateRequest) (res response.InstanceStatusResponse, err error) {
	if viper.GetString("container.provider") == "docker" {
		challenge, err := t.ChallengeRepository.FindById(req.ChallengeId, 1)
		flag := utils.GenerateFlag(challenge.FlagFmt)
		instanceId := uuid.NewString()
		ctn := managers.NewDockerManagerImpl(
			instanceId,
			challenge.Image,
			challenge.ExposedPort,
			flag,
			challenge.FlagEnv,
			challenge.MemoryLimit,
			time.Duration(challenge.Duration)*time.Second)
		port, err := ctn.Setup()
		entry := fmt.Sprintf("%s:%d", viper.GetString("container.docker.public_entry"), port)
		removedAt := time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix()
		err = t.InstanceRepository.Insert(model.Instance{
			InstanceId:  instanceId,
			ChallengeId: req.ChallengeId,
			UserId:      req.UserId,
			Flag:        flag,
			Entry:       entry,
			RemovedAt:   removedAt,
		})
		internal.InstanceMap[instanceId] = ctn
		return response.InstanceStatusResponse{
			InstanceId: instanceId,
			Entry:      entry,
			RemovedAt:  removedAt,
			Status:     "running",
		}, err
	}
	return res, errors.New("创建失败")
}

func (t *InstanceServiceImpl) Status(id string) (rep response.InstanceStatusResponse, err error) {
	rep = response.InstanceStatusResponse{}
	if viper.GetString("container.provider") == "docker" {
		instance, err := t.InstanceRepository.FindById(id)
		if err != nil || internal.InstanceMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		ctn := internal.InstanceMap[id].(*managers.DockerManager)
		status, _ := ctn.GetContainerStatus()
		if status != "removed" {
			rep.InstanceId = id
			rep.Status = status
			rep.Entry = instance.Entry
			rep.RemovedAt = instance.RemovedAt
			return rep, nil
		}
		rep.Status = "removed"
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *InstanceServiceImpl) Renew(id string) (removedAt int64, err error) {
	if viper.GetString("Container.Provider") == "docker" {
		instance, err := t.InstanceRepository.FindById(id)
		if err != nil || internal.InstanceMap[id] == nil {
			return 0, errors.New("实例不存在")
		}
		ctn := internal.InstanceMap[id].(*managers.DockerManager)
		err = ctn.Renew(ctn.Duration)
		instance.RemovedAt = time.Now().Add(ctn.Duration).Unix()
		err = t.InstanceRepository.Update(instance)
		return instance.RemovedAt, err
	}
	return 0, errors.New("续期失败")
}

func (t *InstanceServiceImpl) Remove(id string) (err error) {
	if viper.GetString("container.provider") == "docker" {
		instance, err := t.InstanceRepository.FindById(id)
		if err != nil || internal.InstanceMap[id] == nil {
			return errors.New("实例不存在")
		}
		_ = t.InstanceRepository.Update(instance)
		ctn := internal.InstanceMap[id].(*managers.DockerManager)
		err = ctn.Remove()
		instance.RemovedAt = time.Now().Unix()
		return err
	}
	return errors.New("移除失败")
}

func (t *InstanceServiceImpl) FindById(id string) (rep response.InstanceResponse, err error) {
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
		if req.TeamId != "" && req.GameId != 0 {
			req.UserId = ""
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
