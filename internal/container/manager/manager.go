package manager

import (
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/model"
	"time"
)

type IContainerManager interface {
	Setup() (instances []*model.Instance, err error)
	Status() (status string, err error)
	Duration() (duration time.Duration)
	Remove()
	RemoveAfterDuration() (success bool)
	Renew(duration time.Duration)
	SetPodID(podID uint)
}

func NewContainerManager(images []*model.Image, flag model.Flag, duration time.Duration) IContainerManager {
	switch config.AppCfg().Container.Provider {
	case "docker":
		return NewDockerManager(images, flag, duration)
	case "k8s":
		return NewK8sManager(images, flag, duration)
	}
	return nil
}
