package model

type Env struct {
	ID          uint       `json:"id"`
	Key         string     `gorm:"type:varchar(128);not null;" json:"key"`
	Value       string     `gorm:"type:varchar(128);not null;" json:"value"`
	ChallengeID uint       `gorm:"not null;" json:"challenge_id"`
	Challenge   *Challenge `json:"challenge,omitempty"`
}
