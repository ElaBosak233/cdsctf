package service

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type ContainerService interface {
	Create(req model.Instance)
	Renew(req model.Instance)
	ShutDown(id string)
	FindById(id string) []model.Instance
	FindAll() []model.Instance
}
