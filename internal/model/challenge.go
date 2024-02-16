package model

import (
	"time"
)

// Challenge is the challenge for Jeopardy-style CTF game.
type Challenge struct {
	ID            uint          `json:"id"`                                           // The challenge's id. As primary key.
	Title         string        `gorm:"type:varchar(32);not null;" json:"title"`      // The challenge's title.
	Description   string        `gorm:"type:text;not null;" json:"description"`       // The challenge's description.
	CategoryID    uint          `gorm:"not null;" json:"category_id"`                 // The challenge's category.
	Category      *Category     `json:"category,omitempty"`                           // The challenge's category.
	HasAttachment *bool         `gorm:"not null;default:false" json:"has_attachment"` // Whether the challenge has attachment.
	IsPracticable *bool         `gorm:"not null;default:false" json:"is_practicable"` // Whether the challenge is practicable. (Is the practice field visible.)
	IsDynamic     *bool         `gorm:"default:false" json:"is_dynamic"`              // Whether the challenge is based on dynamic container.
	Difficulty    int64         `gorm:"default:1" json:"difficulty"`                  // The degree of difficulty. (From 1 to 5)
	PracticePts   int64         `gorm:"default:200" json:"practice_pts"`              // The points will be given when the challenge is solved in practice field.
	Duration      int64         `gorm:"default:1800" json:"duration,omitempty"`       // The duration of container maintenance in the initial state. (Seconds)
	CreatedAt     time.Time     `json:"created_at"`                                   // The challenge's creation time.
	UpdatedAt     time.Time     `json:"updated_at"`                                   // The challenge's last update time.
	Flags         []*Flag       `json:"flags,omitempty"`
	Hints         []*Hint       `json:"hints,omitempty"`
	Images        []*Image      `json:"images,omitempty"`
	Submissions   []*Submission `json:"submissions,omitempty"`
}
