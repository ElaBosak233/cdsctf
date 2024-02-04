package entity

import "time"

type Hint struct {
	HintId      int64     `xorm:"'id' pk autoincr"`       // The hint's id.
	ChallengeId int64     `xorm:"'challenge_id' notnull"` // The challenge which the hint belongs to.
	Content     string    `xorm:"'content' text notnull"` // The content of the hint.
	PublishedAt time.Time `xorm:"'published_at' notnull"` // When the hint will be published.
	CreatedAt   time.Time `xorm:"'created_at' created"`   // The hint's creation time.
	UpdatedAt   time.Time `xorm:"'updated_at' updated"`   // The hint's last update time.
}
