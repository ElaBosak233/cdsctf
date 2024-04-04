package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type IHintService interface {
	Create(req request.HintCreateRequest) (err error)
	Update(req request.HintUpdateRequest) (err error)
	Delete(req request.HintDeleteRequest) (err error)
}

type HintService struct {
	hintRepository repository.IHintRepository
}

func NewHintService(appRepository *repository.Repository) IHintService {
	return &HintService{
		hintRepository: appRepository.HintRepository,
	}
}

func (h *HintService) Create(req request.HintCreateRequest) (err error) {
	var hint model.Hint
	_ = mapstructure.Decode(req, &hint)
	_, err = h.hintRepository.Create(hint)
	return err
}

func (h *HintService) Update(req request.HintUpdateRequest) (err error) {
	var hint model.Hint
	_ = mapstructure.Decode(req, &hint)
	_, err = h.hintRepository.Update(hint)
	return err
}

func (h *HintService) Delete(req request.HintDeleteRequest) (err error) {
	var hint model.Hint
	_ = mapstructure.Decode(req, &hint)
	err = h.hintRepository.Delete(hint)
	return err
}
