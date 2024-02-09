package response

import (
	"github.com/elabosak233/pgshub/models/entity"
)

type PodStatusResponse struct {
	ID         int64              `json:"id"`
	Containers []entity.Container `json:"containers"`
	RemovedAt  int64              `json:"removed_at"`
	Status     string             `json:"status"`
}

type PodResponse struct {
	ID          int64              `json:"id"`
	ChallengeID int64              `json:"challenge_id"`
	Containers  []entity.Container `json:"containers"`
	RemovedAt   int64              `json:"removed_at"`
	Status      string             `json:"status"`
}
