package model

import "time"

type GameEvent struct {
	ID          int64      `xorm:"'id' pk autoincr" json:"id"`                               // The game event's id.
	GameID      int64      `xorm:"'game_id' index notnull" json:"game_id"`                   // The game which this event belongs to.
	Type        string     `xorm:"'type' varchar(16) notnull default('notice')" json:"type"` // The game event's type. (Such as "score", "challenge", "notice")
	UserID      string     `xorm:"'user_id' varchar(36) index" json:"user_id"`               // The user who is related to this event.
	TeamID      string     `xorm:"'team_id' index" json:"team_id"`                           // The team which is related to this event.
	ChallengeID string     `xorm:"'challenge_id' varchar(36)" json:"challenge_id"`           // The challenge which is related to this event.
	Score       int64      `xorm:"'score'" json:"score"`                                     // The score of this event. (Only for "score" type)
	Content     string     `xorm:"'content' varchar(128)" json:"content"`                    // The content of this event. (Only for "notice" type)
	CreatedAt   *time.Time `xorm:"'created_at' created" json:"created_at"`                   // The game event's creation time.
	UpdatedAt   *time.Time `xorm:"'updated_at' updated" json:"updated_at"`                   // The game event's last update time.
}
