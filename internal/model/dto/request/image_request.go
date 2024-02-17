package request

import "github.com/elabosak233/cloudsdale/internal/model"

type ImageCreateRequest struct {
	Name        string              `json:"name"`
	CPULimit    float64             `json:"cpu_limit"`
	MemoryLimit int64               `json:"memory_limit"`
	Description string              `json:"description"`
	Ports       []PortCreateRequest `json:"ports"`
	Envs        []model.Env         `json:"envs"`
}
