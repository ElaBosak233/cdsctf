package services

import "github.com/elabosak233/pgshub/internal/models/response"

var InstanceMap = make(map[string]map[string]any)

type InstanceService interface {
	Create(challengeId string) (instanceId string, entry string)
	Status(id string) (rep response.InstanceStatusResponse, error error)
	Renew(id string) error
	Remove(id string) error
	FindById(id string) (rep response.InstanceResponse, err error)
	FindAll() (rep []response.InstanceResponse, err error)
}
