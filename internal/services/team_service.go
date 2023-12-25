package services

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
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
