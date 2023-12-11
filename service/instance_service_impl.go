package service

import (
	"errors"
	"fmt"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/container"
	"github.com/elabosak233/pgshub/model/response"
	"github.com/elabosak233/pgshub/repository"
	"github.com/google/uuid"
	"github.com/spf13/viper"
	"time"
)

type InstanceServiceImpl struct {
	ChallengeRepository repository.ChallengeRepository
}

func NewInstanceServiceImpl(appRepository *repository.AppRepository) InstanceService {
	return &InstanceServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
	}
}

func (t *InstanceServiceImpl) Create(challengeId string) (instanceId string, entry string) {
	if viper.GetString("Container") == "docker" {
		cli, _ := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
		challenge, _ := t.ChallengeRepository.FindById(challengeId)
		ctn := container.NewDockerContainer(
			cli,
			challenge.Image,
			challenge.ExposedPort,
			fmt.Sprintf("DASCTF{%s}", uuid.NewString()),
			challenge.FlagEnv,
			challenge.MemoryLimit,
			time.Duration(challenge.Duration)*time.Second,
		)
		port, _ := ctn.Setup()
		instanceId := uuid.NewString()
		InstanceMap[instanceId] = map[string]any{
			"ctn":         ctn,
			"challengeId": challengeId,
			"entry":       fmt.Sprintf("%s:%d", viper.GetString("Docker.Entry"), port),
			"removeAt":    time.Now().Add(time.Duration(challenge.Duration) * time.Second),
		}
		return instanceId, fmt.Sprintf("%s:%d", viper.GetString("Docker.Entry"), port)
	}
	return "", ""
}

func (t *InstanceServiceImpl) Status(id string) (rep response.InstanceStatusResponse, error error) {
	rep = response.InstanceStatusResponse{}
	if viper.GetString("Container.Provider") == "docker" {
		if InstanceMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		ctn := InstanceMap[id]["ctn"].(*container.DockerContainer)
		status, _ := ctn.GetContainerStatus()
		if status != "removed" {
			rep.InstanceId = id
			rep.Status = status
			rep.Entry = InstanceMap[id]["entry"].(string)
			rep.RemoveAt = InstanceMap[id]["removeAt"].(time.Time)
			return rep, nil
		}
		rep.Status = "removed"
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *InstanceServiceImpl) Renew(id string) error {
	if viper.GetString("Container.Provider") == "docker" {
		ctn := InstanceMap[id]["ctn"].(*container.DockerContainer)
		err := ctn.Renew(ctn.Duration)
		InstanceMap[id]["removeAt"] = time.Now().Add(ctn.Duration)
		return err
	}
	return errors.New("续期失败")
}

func (t *InstanceServiceImpl) Remove(id string) error {
	if viper.GetString("Container.Provider") == "docker" {
		ctn := InstanceMap[id]["ctn"].(*container.DockerContainer)
		err := ctn.Remove()
		return err
	}
	return errors.New("移除失败")
}

func (t *InstanceServiceImpl) FindById(id string) (rep response.InstanceResponse, err error) {
	if viper.GetString("Container.Provider") == "docker" {
		if InstanceMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		status, _ := InstanceMap[id]["ctn"].(*container.DockerContainer).GetContainerStatus()
		rep = response.InstanceResponse{
			InstanceId:  id,
			Entry:       InstanceMap[id]["entry"].(string),
			RemoveAt:    InstanceMap[id]["removeAt"].(time.Time),
			ChallengeId: InstanceMap[id]["challengeId"].(string),
			Status:      status,
		}
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *InstanceServiceImpl) FindAll() (rep []response.InstanceResponse, err error) {
	if viper.GetString("Container.Provider") == "docker" {
		for k, v := range InstanceMap {
			status, _ := v["ctn"].(*container.DockerContainer).GetContainerStatus()
			rep = append(rep, response.InstanceResponse{
				InstanceId:  k,
				Entry:       v["entry"].(string),
				RemoveAt:    v["removeAt"].(time.Time),
				ChallengeId: v["challengeId"].(string),
				Status:      status,
			})
		}
		return rep, nil
	}
	return nil, errors.New("获取失败")
}
