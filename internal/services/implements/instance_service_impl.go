package implements

import (
	"errors"
	"fmt"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/internal/models/response"
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/services/container"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/google/uuid"
	"github.com/spf13/viper"
	"time"
)

type InstanceServiceImpl struct {
	ChallengeRepository repositorys.ChallengeRepository
}

func NewInstanceServiceImpl(appRepository *repositorys.AppRepository) services.InstanceService {
	return &InstanceServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
	}
}

func (t *InstanceServiceImpl) Create(challengeId string) (res response.InstanceStatusResponse, err error) {
	if viper.GetString("Container.Provider") == "docker" {
		cli, _ := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
		challenge, _ := t.ChallengeRepository.FindById(challengeId)
		ctn := container.NewDockerContainer(
			cli,
			challenge.Image,
			challenge.ExposedPort,
			utils.GenerateFlag(challenge.FlagFmt),
			challenge.FlagEnv,
			challenge.MemoryLimit,
			time.Duration(challenge.Duration)*time.Second,
		)
		port, _ := ctn.Setup()
		instanceId := uuid.NewString()
		services.InstanceMap[instanceId] = map[string]any{
			"ctn":         ctn,
			"challengeId": challengeId,
			"entry":       fmt.Sprintf("%s:%d", viper.GetString("container.docker.public_entry"), port),
			"removeAt":    time.Now().Add(time.Duration(challenge.Duration) * time.Second),
		}
		return response.InstanceStatusResponse{
			InstanceId: instanceId,
			Entry:      fmt.Sprintf("%s:%d", viper.GetString("container.docker.public_entry"), port),
			RemoveAt:   time.Now().Add(time.Duration(challenge.Duration) * time.Second).Unix(),
			Status:     "running",
		}, nil
	}
	return res, errors.New("创建失败")
}

func (t *InstanceServiceImpl) Status(id string) (rep response.InstanceStatusResponse, err error) {
	rep = response.InstanceStatusResponse{}
	if viper.GetString("Container.Provider") == "docker" {
		if services.InstanceMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		ctn := services.InstanceMap[id]["ctn"].(*container.DockerContainer)
		status, _ := ctn.GetContainerStatus()
		if status != "removed" {
			rep.InstanceId = id
			rep.Status = status
			rep.Entry = services.InstanceMap[id]["entry"].(string)
			rep.RemoveAt = services.InstanceMap[id]["removeAt"].(time.Time).Unix()
			return rep, nil
		}
		rep.Status = "removed"
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *InstanceServiceImpl) Renew(id string) error {
	if viper.GetString("Container.Provider") == "docker" {
		ctn := services.InstanceMap[id]["ctn"].(*container.DockerContainer)
		err := ctn.Renew(ctn.Duration)
		services.InstanceMap[id]["removeAt"] = time.Now().Add(ctn.Duration)
		return err
	}
	return errors.New("续期失败")
}

func (t *InstanceServiceImpl) Remove(id string) error {
	if viper.GetString("Container.Provider") == "docker" {
		ctn := services.InstanceMap[id]["ctn"].(*container.DockerContainer)
		err := ctn.Remove()
		return err
	}
	return errors.New("移除失败")
}

func (t *InstanceServiceImpl) FindById(id string) (rep response.InstanceResponse, err error) {
	if viper.GetString("Container.Provider") == "docker" {
		if services.InstanceMap[id] == nil {
			return rep, errors.New("实例不存在")
		}
		status, _ := services.InstanceMap[id]["ctn"].(*container.DockerContainer).GetContainerStatus()
		rep = response.InstanceResponse{
			InstanceId:  id,
			Entry:       services.InstanceMap[id]["entry"].(string),
			RemoveAt:    services.InstanceMap[id]["removeAt"].(time.Time).Unix(),
			ChallengeId: services.InstanceMap[id]["challengeId"].(string),
			Status:      status,
		}
		return rep, nil
	}
	return rep, errors.New("获取失败")
}

func (t *InstanceServiceImpl) FindAll() (rep []response.InstanceResponse, err error) {
	if viper.GetString("Container.Provider") == "docker" {
		for k, v := range services.InstanceMap {
			status, _ := v["ctn"].(*container.DockerContainer).GetContainerStatus()
			rep = append(rep, response.InstanceResponse{
				InstanceId:  k,
				Entry:       v["entry"].(string),
				RemoveAt:    v["removeAt"].(time.Time).Unix(),
				ChallengeId: v["challengeId"].(string),
				Status:      status,
			})
		}
		return rep, nil
	}
	return nil, errors.New("获取失败")
}
