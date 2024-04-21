package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type PodStatusResponse struct {
	ID        uint             `json:"id"`
	Challenge *model.Challenge `json:"challenge"`
	Nats      []*model.Nat     `json:"nats"`
	RemovedAt int64            `json:"removed_at"`
	Status    string           `json:"status"`
}

type PodResponse struct {
	ID        uint             `json:"id"`
	Challenge *model.Challenge `json:"challenge"`
	Nats      []*model.Nat     `json:"nats"`
	RemovedAt int64            `json:"removed_at"`
	Status    string           `json:"status"`
}
