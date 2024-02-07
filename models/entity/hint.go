package entity

import "time"

type Hint struct {
	HintID      int64     `xorm:"'id' pk autoincr" json:"id"`                 // The hint's id.
	ChallengeID int64     `xorm:"'challenge_id' notnull" json:"challenge_id"` // The challenge which the hint belongs to.
	GameID      int64     `xorm:"'game_id'" json:"game_id"`                   // The game which the hint belongs to. (If 0, it means the hint is visible in practice field.)
	Content     string    `xorm:"'content' text notnull" json:"content"`      // The content of the hint.
	PublishedAt time.Time `xorm:"'published_at' notnull" json:"published_at"` // When the hint will be published.
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`     // The hint's creation time.
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at"`     // The hint's last update time.
}

func (h *Hint) TableName() string {
	return "hint"
}
