package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type IGroupService interface {
	Find(req request.GroupFindRequest) (groups []model.Group, err error)
	Update(req request.GroupUpdateRequest) (err error)
}

type GroupService struct {
	groupRepository repository.IGroupRepository
}

func NewGroupService(appRepository *repository.Repository) IGroupService {
	return &GroupService{groupRepository: appRepository.GroupRepository}
}

func (g *GroupService) Find(req request.GroupFindRequest) (groups []model.Group, err error) {
	return g.groupRepository.Find(req)
}

func (g *GroupService) Update(req request.GroupUpdateRequest) (err error) {
	return g.groupRepository.Update(req)
}
