package service

import (
	model "github.com/elabosak233/pgshub/model/data"
)

type GroupService interface {
	Create(req model.Group) error
	Update(req model.Group) error
	Delete(id string) error
	FindById(id string) (model.Group, error)
	FindAll() ([]model.Group, error)
}
