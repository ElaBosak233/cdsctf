package service

import (
	"github.com/elabosak233/pgshub/internal/model/request"
	"github.com/elabosak233/pgshub/internal/model/response"
)

type GroupService interface {
	Create(req request.CreateGroupRequest)
	Update(req request.UpdateGroupRequest)
	Delete(id string)
	FindById(id string) response.GroupResponse
	FindAll() []response.GroupResponse
	AddUserToGroup(id string, req request.AddUserToGroupRequest)
}
