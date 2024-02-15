package model

import "time"

type Article struct {
	ID        uint      `json:"id"`                                      // The article's id. As primary key.
	Title     string    `gorm:"type:varchar(50);not null;" json:"title"` // The article's title.
	Summary   string    `gorm:"type:text;not null;" json:"summary"`      // The article's summary.
	Content   string    `gorm:"type:text;not null;" json:"content"`      // The article's content.
	AuthorID  uint      `gorm:"not null;" json:"author_id"`              // The article's author's id.
	CreatedAt time.Time `json:"created_at"`                              // The article's creation time.
	UpdatedAt time.Time `json:"updated_at"`                              // The article's last update time.
}
