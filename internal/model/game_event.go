package model

import "time"

type GameEvent struct {
	ID          int64      `json:"id"`                                                     // The game event's id.
	GameID      int64      `gorm:"index;not null;" json:"game_id"`                         // The game which this event belongs to.
	Type        string     `gorm:"type:varchar(16);not null;default:'notice'" json:"type"` // The game event's type. (Such as "score", "challenge", "notice")
	UserID      uint       `gorm:"index" json:"user_id"`                                   // The user who is related to this event.
	TeamID      uint       `gorm:"index" json:"team_id"`                                   // The team which is related to this event.
	ChallengeID uint       `json:"challenge_id"`                                           // The challenge which is related to this event.
	Score       int64      `json:"score"`                                                  // The score of this event. (Only for "score" type)
	Content     string     `gorm:"type:varchar(128)" json:"content"`                       // The content of this event. (Only for "notice" type)
	CreatedAt   *time.Time `json:"created_at"`                                             // The game event's creation time.
	UpdatedAt   *time.Time `json:"updated_at"`                                             // The game event's last update time.
}
