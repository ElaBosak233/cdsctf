package manager

import (
	"github.com/elabosak233/cloudsdale/internal/extension/config"
	"github.com/elabosak233/cloudsdale/internal/model"
	"time"
)

type IContainerManager interface {
	Setup() (nats []*model.Nat, err error)
	Status() (status string, err error)
	Duration() (duration time.Duration)
	Remove()
	RemoveAfterDuration() (success bool)
	Renew(duration time.Duration)
	SetPodID(podID uint)
}

func NewContainerManager(challenge model.Challenge, flag model.Flag, duration time.Duration) IContainerManager {
	switch config.AppCfg().Container.Provider {
	case "docker":
		return NewDockerManager(challenge, flag, duration)
	case "k8s":
		return NewK8sManager(challenge, flag, duration)
	}
	return nil
}
