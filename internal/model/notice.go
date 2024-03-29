package model

type Notice struct {
	ID          uint   `json:"id"`                                                     // The game event's id.
	GameID      *uint  `gorm:"index;not null;" json:"game_id"`                         // The game which this event belongs to.
	Type        string `gorm:"type:varchar(16);not null;default:'notice'" json:"type"` // The game event's type. (Such as "first_blood", "second_blood", "third_blood", "new_challenge", "new_hint", "normal")
	UserID      *uint  `gorm:"index" json:"user_id"`                                   // The user who is related to this event.
	TeamID      *uint  `gorm:"index" json:"team_id"`                                   // The team which is related to this event.
	ChallengeID *uint  `json:"challenge_id"`                                           // The challenge which is related to this event.
	Content     string `gorm:"type:text" json:"content"`                               // The content of this event. (Only for "notice" type)
	CreatedAt   int64  `gorm:"autoUpdateTime:milli" json:"created_at"`                 // The game event's creation time.
	UpdatedAt   int64  `gorm:"autoUpdateTime:milli" json:"updated_at"`                 // The game event's last update time.
}
