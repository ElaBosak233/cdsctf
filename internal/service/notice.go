package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type INoticeService interface {
	Find(req request.NoticeFindRequest) ([]model.Notice, int64, error)
	Create(req request.NoticeCreateRequest) error
	Update(req request.NoticeUpdateRequest) error
	Delete(req request.NoticeDeleteRequest) error
}

type NoticeService struct {
	noticeRepository repository.INoticeRepository
}

func NewNoticeService(r *repository.Repository) INoticeService {
	return &NoticeService{
		noticeRepository: r.NoticeRepository,
	}
}

func (n *NoticeService) Find(req request.NoticeFindRequest) ([]model.Notice, int64, error) {
	notices, total, err := n.noticeRepository.Find(req)
	return notices, total, err
}

func (n *NoticeService) Create(req request.NoticeCreateRequest) error {
	var notice model.Notice
	_ = mapstructure.Decode(req, &notice)
	_, err := n.noticeRepository.Create(notice)
	return err
}

func (n *NoticeService) Update(req request.NoticeUpdateRequest) error {
	var notice model.Notice
	_ = mapstructure.Decode(req, &notice)
	_, err := n.noticeRepository.Update(notice)
	return err
}

func (n *NoticeService) Delete(req request.NoticeDeleteRequest) error {
	var notice model.Notice
	_ = mapstructure.Decode(req, &notice)
	err := n.noticeRepository.Delete(notice)
	return err
}
