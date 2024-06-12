package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IWebhookRepository interface {
	Create(webhook model.Webhook) (model.Webhook, error)
	Update(webhook model.Webhook) error
	Delete(webhook model.Webhook) error
}

type WebhookRepository struct {
	db *gorm.DB
}

func NewWebhookRepository(db *gorm.DB) IWebhookRepository {
	return &WebhookRepository{db: db}
}

func (t *WebhookRepository) Create(webhook model.Webhook) (model.Webhook, error) {
	result := t.db.Table("webhooks").Create(&webhook)
	return webhook, result.Error
}

func (t *WebhookRepository) Update(webhook model.Webhook) error {
	result := t.db.Table("webhooks").Model(&webhook).Updates(&webhook)
	return result.Error
}

func (t *WebhookRepository) Delete(webhook model.Webhook) error {
	result := t.db.Table("webhooks").Where("id = ?", webhook.ID).Delete(&webhook)
	return result.Error
}
