package model

import (
	"github.com/elabosak233/cloudsdale/internal/broadcast"
	"gorm.io/gorm"
)

type Notice struct {
	ID          uint       `json:"id"`                                                               // The game event's id.
	Type        string     `gorm:"type:varchar(16);not null;default:'notice'" json:"type,omitempty"` // The game event's type. (Such as "first_blood", "second_blood", "third_blood", "new_challenge", "new_hint", "normal")
	GameID      *uint      `gorm:"index;not null;" json:"game_id,omitempty"`                         // The game which this event belongs to.
	Game        *Game      `json:"game,omitempty"`                                                   // The game which this event belongs to.
	UserID      *uint      `gorm:"index" json:"user_id,omitempty"`                                   // The user who is related to this event.
	User        *User      `json:"user,omitempty"`                                                   // The user who is related to this event.
	TeamID      *uint      `gorm:"index" json:"team_id,omitempty"`                                   // The team which is related to this event.
	Team        *Team      `json:"team,omitempty"`                                                   // The team which is related to this event.
	ChallengeID *uint      `json:"challenge_id,omitempty"`                                           // The challenge which is related to this event.
	Challenge   *Challenge `json:"challenge,omitempty"`                                              // The challenge which is related to this event.
	Content     string     `gorm:"type:text" json:"content,omitempty"`                               // The content of this event. (Only for "notice" type)
	CreatedAt   int64      `gorm:"autoUpdateTime:milli" json:"created_at,omitempty"`                 // The game event's creation time.
	UpdatedAt   int64      `gorm:"autoUpdateTime:milli" json:"updated_at,omitempty"`                 // The game event's last update time.
}

func (n *Notice) AfterCreate(db *gorm.DB) (err error) {
	result := db.Table("notices").
		Preload("User", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "username", "nickname", "email"})
		}).
		Preload("Team", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "name", "email"})
		}).
		Preload("Challenge", func(db *gorm.DB) *gorm.DB {
			return db.Select([]string{"id", "title"})
		}).
		First(n, n.ID)
	broadcast.SendGameMsg(*(n.GameID), n)
	return result.Error
}
