package response

import "time"

type InstanceStatusResponse struct {
	InstanceId string    `json:"id"`
	Entry      string    `json:"entry"`
	RemoveAt   time.Time `json:"remove_at"`
	Status     string    `json:"status"`
}

type InstanceResponse struct {
	InstanceId  string    `json:"id"`
	Entry       string    `json:"entry"`
	RemoveAt    time.Time `json:"remove_at"`
	Status      string    `json:"status"`
	ChallengeId string    `json:"challenge_id"`
}
