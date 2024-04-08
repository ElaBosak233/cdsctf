package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type PodStatusResponse struct {
	ID        uint             `json:"id"`
	Container *model.Container `json:"container"`
	RemovedAt int64            `json:"removed_at"`
	Status    string           `json:"status"`
}

type PodResponse struct {
	ID          uint             `json:"id"`
	ChallengeID uint             `json:"challenge_id"`
	Container   *model.Container `json:"container"`
	RemovedAt   int64            `json:"removed_at"`
	Status      string           `json:"status"`
}
