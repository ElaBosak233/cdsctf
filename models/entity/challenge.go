package entity

import "time"

// Challenge is the challenge for Jeopardy-style CTF game.
type Challenge struct {
	ChallengeID   int64        `xorm:"'id' pk autoincr" json:"id"`                                         // The challenge's id. As primary key.
	Title         string       `xorm:"'title' nvarchar(32) notnull" json:"title"`                          // The challenge's title.
	Description   string       `xorm:"'description' text notnull" json:"description"`                      // The challenge's description.
	CategoryID    int64        `xorm:"'category_id' notnull" json:"category_id"`                           // The challenge's category.
	HasAttachment *bool        `xorm:"'has_attachment' bool notnull default(false)" json:"has_attachment"` // Whether the challenge has attachment.
	IsPracticable *bool        `xorm:"'is_practicable' bool notnull default(false)" json:"is_practicable"` // Whether the challenge is practicable. (Is the practice field visible.)
	IsDynamic     *bool        `xorm:"'is_dynamic' bool default(false)" json:"is_dynamic"`                 // Whether the challenge is based on dynamic container.
	Difficulty    int64        `xorm:"'difficulty' default(1)" json:"difficulty"`                          // The degree of difficulty. (From 1 to 5)
	PracticePts   int64        `xorm:"'practice_pts' default(200) notnull" json:"practice_pts"`            // The points will be given when the challenge is solved in practice field.
	Duration      int64        `xorm:"'duration' default(1800)" json:"duration,omitempty"`                 // The duration of container maintenance in the initial state. (Seconds)
	Category      Category     `xorm:"-" json:"category"`
	Flags         []Flag       `xorm:"-" json:"flags"`
	Hints         []Hint       `xorm:"-" json:"hints"`
	Images        []Image      `xorm:"-" json:"images"`
	Submissions   []Submission `xorm:"-" json:"submissions"`
	Pts           int64        `xorm:"-" json:"pts"`
	IsSolved      bool         `xorm:"-" json:"is_solved"`
	CreatedAt     time.Time    `xorm:"'created_at' created" json:"created_at"` // The challenge's creation time.
	UpdatedAt     time.Time    `xorm:"'updated_at' updated" json:"updated_at"` // The challenge's last update time.
}

func (c *Challenge) TableName() string {
	return "challenge"
}
