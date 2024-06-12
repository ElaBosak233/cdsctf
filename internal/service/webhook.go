package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type IWebhookService interface {
	Create(webhook model.Webhook) (model.Webhook, error)
	Update(webhook model.Webhook) error
	Delete(webhook model.Webhook) error
}

type WebhookService struct {
	webhookRepository repository.IWebhookRepository
	gameRepository    repository.IGameRepository
}

func NewWebhookService(r *repository.Repository) IWebhookService {
	return &WebhookService{
		webhookRepository: r.WebhookRepository,
		gameRepository:    r.GameRepository,
	}
}

func (t *WebhookService) Create(webhook model.Webhook) (model.Webhook, error) {
	return model.Webhook{}, nil
}

func (t *WebhookService) Update(webhook model.Webhook) error {
	return nil
}

func (t *WebhookService) Delete(webhook model.Webhook) error {
	return nil
}
