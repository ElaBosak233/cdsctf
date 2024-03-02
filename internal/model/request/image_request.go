package request

import "github.com/elabosak233/cloudsdale/internal/model"

type ImageCreateRequest struct {
	ChallengeID uint          `json:"-"`
	Name        string        `json:"name"`
	CPULimit    float64       `json:"cpu_limit"`
	MemoryLimit int64         `json:"memory_limit"`
	Description string        `json:"description"`
	Ports       []*model.Port `json:"ports"`
	Envs        []*model.Port `json:"envs"`
}

type ImageUpdateRequest struct {
	ID          uint          `json:"-"`
	ChallengeID uint          `json:"-"`
	Name        string        `json:"name"`
	CPULimit    float64       `json:"cpu_limit"`
	MemoryLimit int64         `json:"memory_limit"`
	Description string        `json:"description"`
	Ports       []*model.Port `json:"ports"`
	Envs        []*model.Port `json:"envs"`
}

type ImageDeleteRequest struct {
	ID          uint `json:"-"`
	ChallengeID uint `json:"-"`
}
