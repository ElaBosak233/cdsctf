package model

type Hint struct {
	ID          uint       `json:"id"`                                     // The hint's id.
	ChallengeID uint       `gorm:"not null;" json:"challenge_id"`          // The challenge which the hint belongs to.
	Challenge   *Challenge `json:"challenge"`                              // The challenge which the hint belongs to.
	Content     string     `gorm:"type:text;not null;" json:"content"`     // The content of the hint.
	PublishedAt int64      `gorm:"not null;" json:"published_at"`          // When the hint will be published.
	CreatedAt   int64      `gorm:"autoUpdateTime:milli" json:"created_at"` // The hint's creation time.
	UpdatedAt   int64      `gorm:"autoUpdateTime:milli" json:"updated_at"` // The hint's last update time.
}
