package service

type IMediaService interface {
}

type MediaService struct{}

func NewMediaService() IMediaService {
	return &MediaService{}
}
