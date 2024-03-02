package model

import "time"

type Hint struct {
	ID          uint       `json:"id"`                                 // The hint's id.
	ChallengeID uint       `gorm:"not null;" json:"challenge_id"`      // The challenge which the hint belongs to.
	Challenge   *Challenge `json:"challenge"`                          // The challenge which the hint belongs to.
	Content     string     `gorm:"type:text;not null;" json:"content"` // The content of the hint.
	PublishedAt int64      `gorm:"not null;" json:"published_at"`      // When the hint will be published.
	CreatedAt   *time.Time `json:"created_at"`                         // The hint's creation time.
	UpdatedAt   *time.Time `json:"updated_at"`                         // The hint's last update time.
}
