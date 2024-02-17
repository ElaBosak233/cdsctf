package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type PodStatusResponse struct {
	ID        uint              `json:"id"`
	Instances []*model.Instance `json:"instances"`
	RemovedAt int64             `json:"removed_at"`
	Status    string            `json:"status"`
}

type PodResponse struct {
	ID          uint              `json:"id"`
	ChallengeID uint              `json:"challenge_id"`
	Instances   []*model.Instance `json:"instances"`
	RemovedAt   int64             `json:"removed_at"`
	Status      string            `json:"status"`
}
