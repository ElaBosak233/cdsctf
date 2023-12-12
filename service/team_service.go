package service

import (
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/model/response"
)

type TeamService interface {
	Create(req request.TeamCreateRequest) error
	Update(req request.TeamUpdateRequest) error
	Delete(id string) error
	Join(req request.TeamJoinRequest) error
	Quit(req request.TeamQuitRequest) error
	FindById(id string) (res response.TeamResponse, err error)
	FindAll() (reses []response.TeamResponse, err error)
}
