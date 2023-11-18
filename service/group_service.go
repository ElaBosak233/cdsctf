package service

import (
	request2 "github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/model/response"
)

type GroupService interface {
	Create(req request2.CreateGroupRequest)
	Update(req request2.UpdateGroupRequest)
	Delete(id string)
	FindById(id string) response.GroupResponse
	FindAll() []response.GroupResponse
	AddUserToGroup(id string, req request2.AddUserToGroupRequest)
}
