package model

type Container struct {
	ID          uint       `json:"id,omitempty"`
	ChallengeID uint       `gorm:"not null" json:"challenge_id,omitempty"`
	Challenge   *Challenge `json:"challenge,omitempty"`
	PodID       uint       `gorm:"not null" json:"pod_id,omitempty"`
	Pod         *Pod       `json:"pod,omitempty"`
	Nats        []*Nat     `json:"nats,omitempty"`
}
