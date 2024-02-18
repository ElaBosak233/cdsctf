package manager

import (
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/model"
	"time"
)

type ContainerManager interface {
	Setup() (instances []*model.Instance, err error)
	GetContainerStatus() (status string, err error)
	RemoveAfterDuration() (success bool)
	Remove()
	Renew(duration time.Duration)
	SetPodID(podID uint)
	GetDuration() (duration time.Duration)
}

func NewContainerManager(images []*model.Image, flag model.Flag, duration time.Duration) ContainerManager {
	switch config.AppCfg().Container.Provider {
	case "docker":
		return NewDockerManager(images, flag, duration)
	case "k8s":
		return NewK8sManager(images, flag, duration)
	}
	return nil
}
