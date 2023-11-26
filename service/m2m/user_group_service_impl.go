package m2m

import (
	"errors"
	model "github.com/elabosak233/pgshub/model/data/m2m"
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/repository/m2m"
)

type UserGroupServiceImpl struct {
	UserGroupRepository m2m.UserGroupRepository
	GroupRepository     repository.GroupRepository
	UserRepository      repository.UserRepository
}

func NewUserServiceImpl(appRepository repository.AppRepository) UserGroupService {
	return &UserGroupServiceImpl{
		UserGroupRepository: appRepository.UserGroupRepository,
		GroupRepository:     appRepository.GroupRepository,
		UserRepository:      appRepository.UserRepository,
	}
}

// Create implements UserService
func (t *UserGroupServiceImpl) Create(req model.UserGroup) error {
	userGroupModel := model.UserGroup{
		UserId:  req.UserId,
		GroupId: req.GroupId,
	}
	existUser, err := t.UserRepository.FindById(userGroupModel.UserId)
	if existUser.UserId == "" {
		return errors.New("用户不存在")
	}
	existGroup, err := t.GroupRepository.FindById(userGroupModel.GroupId)
	if existGroup.GroupId == "" {
		return errors.New("用户组不存在")
	}
	exist, err := t.UserGroupRepository.Exist(userGroupModel)
	if err != nil || exist {
		return errors.New("请勿重复添加")
	}
	err = t.UserGroupRepository.Insert(userGroupModel)
	return err
}

// Delete implements UserService
func (t *UserGroupServiceImpl) Delete(req model.UserGroup) error {
	userGroupModel := model.UserGroup{
		UserId:  req.UserId,
		GroupId: req.GroupId,
	}
	exist, err := t.UserGroupRepository.Exist(userGroupModel)
	if err != nil || !exist {
		return errors.New("记录不存在")
	}
	err = t.UserGroupRepository.Delete(req)
	return err
}

// FindAll implements UserService
func (t *UserGroupServiceImpl) FindAll() ([]model.UserGroup, error) {
	userGroupData, err := t.UserGroupRepository.FindAll()
	if err != nil {
		return nil, errors.New("查询失败")
	}
	return userGroupData, nil
}

// FindByUserId implements UserService
func (t *UserGroupServiceImpl) FindByUserId(userId string) ([]model.UserGroup, error) {
	userGroupData, err := t.UserGroupRepository.FindByUserId(userId)
	if err != nil {
		return nil, errors.New("查询失败")
	}
	return userGroupData, nil
}

func (t *UserGroupServiceImpl) FindByGroupId(groupId string) ([]model.UserGroup, error) {
	userGroupData, err := t.UserGroupRepository.FindByGroupId(groupId)
	if err != nil {
		return nil, errors.New("查询失败")
	}
	return userGroupData, nil
}
