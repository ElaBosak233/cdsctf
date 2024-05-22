package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type INoticeService interface {
	Find(req request.NoticeFindRequest) (notices []model.Notice, total int64, err error)
	Create(req request.NoticeCreateRequest) (err error)
	Update(req request.NoticeUpdateRequest) (err error)
	Delete(req request.NoticeDeleteRequest) (err error)
}

type NoticeService struct {
	noticeRepository repository.INoticeRepository
}

func NewNoticeService(appRepository *repository.Repository) INoticeService {
	return &NoticeService{
		noticeRepository: appRepository.NoticeRepository,
	}
}

func (n *NoticeService) Find(req request.NoticeFindRequest) (notices []model.Notice, total int64, err error) {
	notices, total, err = n.noticeRepository.Find(req)
	return notices, total, err
}

func (n *NoticeService) Create(req request.NoticeCreateRequest) (err error) {
	var notice model.Notice
	_ = mapstructure.Decode(req, &notice)
	_, err = n.noticeRepository.Create(notice)
	return err
}

func (n *NoticeService) Update(req request.NoticeUpdateRequest) (err error) {
	var notice model.Notice
	_ = mapstructure.Decode(req, &notice)
	_, err = n.noticeRepository.Update(notice)
	return err
}

func (n *NoticeService) Delete(req request.NoticeDeleteRequest) (err error) {
	var notice model.Notice
	_ = mapstructure.Decode(req, &notice)
	err = n.noticeRepository.Delete(notice)
	return
}
