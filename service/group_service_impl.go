package service

import (
	"errors"
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/repository"
	"github.com/google/uuid"
)

type GroupServiceImpl struct {
	GroupRepository repository.GroupRepository
	UserRepository  repository.UserRepository
}

func NewGroupServiceImpl(appRepository *repository.AppRepository) GroupService {
	return &GroupServiceImpl{
		GroupRepository: appRepository.GroupRepository,
		UserRepository:  appRepository.UserRepository,
	}
}

// Create implements UserService
func (t *GroupServiceImpl) Create(req model.Group) error {
	groupModel := model.Group{
		GroupId: uuid.NewString(),
		Name:    req.Name,
	}
	t.GroupRepository.Insert(groupModel)
	return nil
}

// Delete implements UserService
func (t *GroupServiceImpl) Delete(id string) error {
	t.GroupRepository.Delete(id)
	return nil
}

// FindAll implements UserService
func (t *GroupServiceImpl) FindAll() ([]model.Group, error) {
	result := t.GroupRepository.FindAll()
	var groups []model.Group
	for _, value := range result {
		group := model.Group{
			GroupId: value.GroupId,
			Name:    value.Name,
		}
		groups = append(groups, group)
	}

	return groups, nil
}

// FindById implements UserService
func (t *GroupServiceImpl) FindById(id string) (model.Group, error) {
	groupData, err := t.GroupRepository.FindById(id)
	if err != nil || groupData.GroupId == "" {
		return groupData, errors.New("用户组不存在")
	}
	group := model.Group{
		GroupId: groupData.GroupId,
		Name:    groupData.Name,
	}
	return group, nil
}

// Update implements UserService
func (t *GroupServiceImpl) Update(req model.Group) error {
	groupData, err := t.GroupRepository.FindById(req.GroupId)
	if err != nil || groupData.GroupId == "" {
		return errors.New("用户组不存在")
	}
	groupData.Name = req.Name
	t.GroupRepository.Update(groupData)
	return nil
}
