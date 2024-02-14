package response

import (
	"github.com/elabosak233/pgshub/internal/model"
)

type PodStatusResponse struct {
	ID        int64            `json:"id"`
	Instances []model.Instance `json:"instances"`
	RemovedAt int64            `json:"removed_at"`
	Status    string           `json:"status"`
}

type PodResponse struct {
	ID          int64            `json:"id"`
	ChallengeID int64            `json:"challenge_id"`
	Instances   []model.Instance `json:"instances"`
	RemovedAt   int64            `json:"removed_at"`
	Status      string           `json:"status"`
}
