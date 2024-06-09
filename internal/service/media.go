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
	// SaveGamePoster will save the game poster to the media folder with the game id as the folder name.
	SaveGamePoster(id uint, fileHeader *multipart.FileHeader) error

	// DeleteGamePoster will delete the game poster from the media folder with the game id as the folder name.
	DeleteGamePoster(id uint) error

	// SaveUserAvatar will save the user avatar to the media folder with the user id as the folder name.
	SaveUserAvatar(id uint, fileHeader *multipart.FileHeader) error

	// DeleteUserAvatar will delete the user avatar from the media folder with the user id as the folder name.
	DeleteUserAvatar(id uint) error

	// SaveTeamAvatar will save the team avatar to the media folder with the team id as the folder name.
	SaveTeamAvatar(id uint, fileHeader *multipart.FileHeader) error

	// DeleteTeamAvatar will delete the team avatar from the media folder with the team id as the folder name.
	DeleteTeamAvatar(id uint) error

	// SaveChallengeAttachment will save the challenge attachment to the media folder with the challenge id as the folder name.
	SaveChallengeAttachment(id uint, fileHeader *multipart.FileHeader) error

	// DeleteChallengeAttachment will delete the challenge attachment from the media folder with the challenge id as the folder name.
	DeleteChallengeAttachment(id uint) error
}

type MediaService struct{}

func NewMediaService() IMediaService {
	return &MediaService{}
}

func (m *MediaService) SaveChallengeAttachment(id uint, fileHeader *multipart.FileHeader) error {
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

func (m *MediaService) DeleteChallengeAttachment(id uint) error {
	p := path.Join(utils.MediaPath, "challenges", fmt.Sprintf("%d", id))
	return os.RemoveAll(p)
}

func (m *MediaService) SaveGamePoster(id uint, fileHeader *multipart.FileHeader) error {
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

func (m *MediaService) DeleteGamePoster(id uint) error {
	p := path.Join(utils.MediaPath, "games", fmt.Sprintf("%d", id), "poster")
	return os.RemoveAll(p)
}

func (m *MediaService) SaveUserAvatar(id uint, fileHeader *multipart.FileHeader) error {
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

func (m *MediaService) DeleteUserAvatar(id uint) error {
	p := path.Join(utils.MediaPath, "users", fmt.Sprintf("%d", id))
	return os.RemoveAll(p)
}

func (m *MediaService) SaveTeamAvatar(id uint, fileHeader *multipart.FileHeader) error {
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

func (m *MediaService) DeleteTeamAvatar(id uint) error {
	p := path.Join(utils.MediaPath, "teams", fmt.Sprintf("%d", id))
	return os.RemoveAll(p)
}
