package service

import (
	"github.com/elabosak233/pgshub/internal/model/data"
	"github.com/elabosak233/pgshub/internal/model/request"
	"github.com/elabosak233/pgshub/internal/model/response"
	"github.com/elabosak233/pgshub/internal/repository"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/go-playground/validator/v10"
	"github.com/satori/go.uuid"
)

type GroupServiceImpl struct {
	GroupRepository repository.GroupRepository
	UserRepository  repository.UserRepository
	Validate        *validator.Validate
}

func NewGroupServiceImpl(appRepository repository.AppRepository, validate *validator.Validate) GroupService {
	return &GroupServiceImpl{
		GroupRepository: appRepository.GroupRepository,
		UserRepository:  appRepository.UserRepository,
		Validate:        validate,
	}
}

// Create implements UserService
func (t *GroupServiceImpl) Create(req request.CreateGroupRequest) {
	err := t.Validate.Struct(req)
	utils.ErrorPanic(err)
	groupModel := data.Group{
		Id:   uuid.NewV4().String(),
		Name: req.Name,
	}
	t.GroupRepository.Insert(groupModel)
}

// Delete implements UserService
func (t *GroupServiceImpl) Delete(id string) {
	t.GroupRepository.Delete(id)
}

// FindAll implements UserService
func (t *GroupServiceImpl) FindAll() []response.GroupResponse {
	result := t.GroupRepository.FindAll()

	var groups []response.GroupResponse
	for _, value := range result {
		group := response.GroupResponse{
			Id:      value.Id,
			Name:    value.Name,
			UserIds: value.UserIds,
		}
		groups = append(groups, group)
	}

	return groups
}

// FindById implements UserService
func (t *GroupServiceImpl) FindById(id string) response.GroupResponse {
	groupData, err := t.GroupRepository.FindById(id)
	utils.ErrorPanic(err)

	group := response.GroupResponse{
		Id:      groupData.Id,
		Name:    groupData.Name,
		UserIds: groupData.UserIds,
	}
	return group
}

// Update implements UserService
func (t *GroupServiceImpl) Update(req request.UpdateGroupRequest) {
	groupData, err := t.GroupRepository.FindById(req.Id)
	utils.ErrorPanic(err)
	groupData.Name = req.Name
	t.GroupRepository.Update(groupData)
}

func (t *GroupServiceImpl) AddUserToGroup(id string, req request.AddUserToGroupRequest) {
	user, err := t.UserRepository.FindById(req.Id)
	if err != nil || user.Id == "" {
		utils.ErrorPanic(err)
		return
	}
	group, err := t.GroupRepository.FindById(id)
	if err != nil || group.Id == "" {
		utils.ErrorPanic(err)
		return
	}
	if !contains(group.UserIds, user.Id) {
		group.UserIds = append(group.UserIds, user.Id)
		t.GroupRepository.Update(group)
	}

	if !contains(user.GroupIds, group.Id) {
		user.GroupIds = append(user.GroupIds, group.Id)
		t.UserRepository.Update(user)
	}
}

func contains(slice []string, element string) bool {
	for _, e := range slice {
		if e == element {
			return true
		}
	}
	return false
}
