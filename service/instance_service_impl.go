package service

import (
	"errors"
	"fmt"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/container"
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
func (t *InstanceServiceImpl) Create(challengeId string) (instanceId string) {
	if viper.GetString("Container") == "docker" {
		cli, _ := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
		challenge, _ := t.ChallengeRepository.FindById(challengeId)
		ctn := container.NewDockerContainer(
			cli,
			challenge.ImageName,
			challenge.InsidePort,
			fmt.Sprintf("DASCTF{%s}", uuid.NewString()),
			challenge.FlagEnv,
			challenge.MemoryLimit,
			time.Duration(challenge.Duration)*time.Second,
		)
		_ = ctn.Setup()
		instanceId := uuid.NewString()
		InstanceMap[instanceId] = ctn
		return instanceId
	}
	return ""
}

func (t *InstanceServiceImpl) Status(id string) (status string, error error) {
	if viper.GetString("Container") == "docker" {
		if InstanceMap[id] == nil {
			return "", errors.New("实例不存在")
		}
		ctn := InstanceMap[id].(*container.DockerContainer)
		status, _ = ctn.GetContainerStatus()
		return status, nil
	}
	return "", errors.New("获取失败")
}

func (t *InstanceServiceImpl) Renew(id string) error {
	if viper.GetString("Container") == "docker" {
		ctn := InstanceMap[id].(*container.DockerContainer)
		err := ctn.Renew(ctn.Duration)
		return err
	}
	return errors.New("续期失败")
}

func (t *InstanceServiceImpl) Remove(id string) error {
	if viper.GetString("Container") == "docker" {
		ctn := InstanceMap[id].(*container.DockerContainer)
		err := ctn.Remove()
		return err
	}
	return errors.New("移除失败")
}
