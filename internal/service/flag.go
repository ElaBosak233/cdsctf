package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type IFlagService interface {
	Create(req request.FlagCreateRequest) error
	Update(req request.FlagUpdateRequest) error
	Delete(req request.FlagDeleteRequest) error
}

type FlagService struct {
	flagRepository repository.IFlagRepository
}

func NewFlagService(r *repository.Repository) IFlagService {
	return &FlagService{
		flagRepository: r.FlagRepository,
	}
}

func (f *FlagService) Create(req request.FlagCreateRequest) error {
	var flag model.Flag
	_ = mapstructure.Decode(req, &flag)
	_, err := f.flagRepository.Create(flag)
	return err
}

func (f *FlagService) Update(req request.FlagUpdateRequest) error {
	var flag model.Flag
	_ = mapstructure.Decode(req, &flag)
	_, err := f.flagRepository.Update(flag)
	return err
}

func (f *FlagService) Delete(req request.FlagDeleteRequest) error {
	var flag model.Flag
	_ = mapstructure.Decode(req, &flag)
	err := f.flagRepository.Delete(flag)
	return err
}
