package service

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"io"
	"mime/multipart"
	"os"
	"path"
)

type IMediaService interface {
	SaveGamePoster(id uint, fileHeader *multipart.FileHeader) (err error)
	DeleteGamePoster(id uint) (err error)
	SaveUserAvatar(id uint, fileHeader *multipart.FileHeader) (err error)
	DeleteUserAvatar(id uint) (err error)
	SaveTeamAvatar(id uint, fileHeader *multipart.FileHeader) (err error)
	DeleteTeamAvatar(id uint) (err error)
	FindChallengeAttachment(id uint) (filename string, size int64, err error)
	SaveChallengeAttachment(id uint, fileHeader *multipart.FileHeader) (err error)
	DeleteChallengeAttachment(id uint) (err error)
}

type MediaService struct{}

func NewMediaService() IMediaService {
	return &MediaService{}
}

func (m *MediaService) FindChallengeAttachment(id uint) (filename string, size int64, err error) {
	p := path.Join(utils.MediaPath, "challenges", fmt.Sprintf("%d", id))
	files, err := os.ReadDir(p)
	for _, file := range files {
		filename = file.Name()
		info, _ := file.Info()
		size = info.Size()
		break
	}
	return filename, size, err
}

func (m *MediaService) SaveChallengeAttachment(id uint, fileHeader *multipart.FileHeader) (err error) {
	file, err := fileHeader.Open()
	defer func(file multipart.File) {
		_ = file.Close()
	}(file)
	data, err := io.ReadAll(file)
	p := path.Join(utils.MediaPath, "challenges", fmt.Sprintf("%d", id), fileHeader.Filename)
	err = m.DeleteChallengeAttachment(id)
	dir := path.Dir(p)
	if _, err = os.Stat(dir); os.IsNotExist(err) {
		if err = os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	err = os.WriteFile(p, data, 0644)
	return err
}

func (m *MediaService) DeleteChallengeAttachment(id uint) (err error) {
	p := path.Join(utils.MediaPath, "challenges", fmt.Sprintf("%d", id))
	return os.RemoveAll(p)
}

func (m *MediaService) SaveGamePoster(id uint, fileHeader *multipart.FileHeader) (err error) {
	file, err := fileHeader.Open()
	defer func(file multipart.File) {
		_ = file.Close()
	}(file)
	data, err := io.ReadAll(file)
	p := path.Join(utils.MediaPath, "games", fmt.Sprintf("%d", id), "poster", fileHeader.Filename)
	err = m.DeleteGamePoster(id)
	dir := path.Dir(p)
	if _, err = os.Stat(dir); os.IsNotExist(err) {
		if err = os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	err = os.WriteFile(p, data, 0644)
	return err
}

func (m *MediaService) DeleteGamePoster(id uint) (err error) {
	p := path.Join(utils.MediaPath, "games", fmt.Sprintf("%d", id), "poster")
	return os.RemoveAll(p)
}

func (m *MediaService) SaveUserAvatar(id uint, fileHeader *multipart.FileHeader) (err error) {
	file, err := fileHeader.Open()
	defer func(file multipart.File) {
		_ = file.Close()
	}(file)
	data, err := io.ReadAll(file)
	p := path.Join(utils.MediaPath, "users", fmt.Sprintf("%d", id), fileHeader.Filename)
	err = m.DeleteUserAvatar(id)
	dir := path.Dir(p)
	if _, err = os.Stat(dir); os.IsNotExist(err) {
		if err = os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	err = os.WriteFile(p, data, 0644)
	return err
}

func (m *MediaService) DeleteUserAvatar(id uint) (err error) {
	p := path.Join(utils.MediaPath, "users", fmt.Sprintf("%d", id))
	return os.RemoveAll(p)
}

func (m *MediaService) SaveTeamAvatar(id uint, fileHeader *multipart.FileHeader) (err error) {
	file, err := fileHeader.Open()
	defer func(file multipart.File) {
		_ = file.Close()
	}(file)
	data, err := io.ReadAll(file)
	p := path.Join(utils.MediaPath, "teams", fmt.Sprintf("%d", id), fileHeader.Filename)
	err = m.DeleteTeamAvatar(id)
	dir := path.Dir(p)
	if _, err = os.Stat(dir); os.IsNotExist(err) {
		if err = os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	err = os.WriteFile(p, data, 0644)
	return err
}

func (m *MediaService) DeleteTeamAvatar(id uint) (err error) {
	p := path.Join(utils.MediaPath, "teams", fmt.Sprintf("%d", id))
	return os.RemoveAll(p)
}
