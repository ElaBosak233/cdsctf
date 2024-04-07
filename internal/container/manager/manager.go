package manager

import (
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
	return NewDockerManager(images, flag, duration)
}
