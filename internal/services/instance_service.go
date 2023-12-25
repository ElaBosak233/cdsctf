package services

import "github.com/elabosak233/pgshub/internal/models/response"

var InstanceMap = make(map[string]map[string]any)

type InstanceService interface {
	Create(challengeId string) (res response.InstanceStatusResponse, err error)
	Status(id string) (rep response.InstanceStatusResponse, err error)
	Renew(id string) error
	Remove(id string) error
	FindById(id string) (rep response.InstanceResponse, err error)
	FindAll() (rep []response.InstanceResponse, err error)
}
