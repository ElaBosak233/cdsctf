package redis

import "time"

type Instance struct {
	InstanceId  string    `json:"id"`
	ChallengeId string    `json:"challenge_id"`
	CreatedAt   time.Time `json:"created_at"`
}
