package service

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type ContainerService interface {
	Create(req model.Container)
	Renew(req model.Container)
	ShutDown(id string)
	FindById(id string) []model.Container
	FindAll() []model.Container
}
