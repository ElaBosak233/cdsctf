package service

import (
	req "github.com/elabosak233/pgshub/model/request/account"
	"github.com/elabosak233/pgshub/model/response"
)

type ContainerService interface {
	Create(req req.CreateGroupRequest)
	Renew(req req.UpdateGroupRequest)
	ShutDown(id string)
	FindById(id string) response.GroupResponse
	FindAll() []response.GroupResponse
}
