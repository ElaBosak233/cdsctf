package response

import "time"

type InstanceStatusResponse struct {
	InstanceID int64     `json:"id"`
	Entry      string    `json:"entry"`
	RemovedAt  time.Time `json:"removed_at"`
	Status     string    `json:"status"`
}

type InstanceResponse struct {
	InstanceID  int64     `json:"id"`
	Entry       string    `json:"entry"`
	RemovedAt   time.Time `json:"removed_at"`
	Status      string    `json:"status"`
	ChallengeID int64     `json:"challenge_id"`
}
