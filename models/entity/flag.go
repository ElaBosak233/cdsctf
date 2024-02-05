package entity

import "time"

type Flag struct {
	FlagID      int64     `xorm:"'id' pk autoincr" json:"id"`                               // The flag id.
	Type        string    `xorm:"'type' varchar(16) notnull default('static')" json:"type"` // The flag type. ("static"/"dynamic"/"pattern")
	Content     string    `xorm:"'content' varchar(255)" json:"content"`                    // The flag content. Maybe a string or a regex, or the placeholder for dynamic challenges. (Such as "flag{friendsh1p_1s_magic}" or "flag{[a-zA-Z]{5}}" or "flag{[UUID]}")
	Env         string    `xorm:"'env' varchar(16)" json:"env"`                             // The environment variable which is used to be injected with the flag.
	ChallengeID int64     `xorm:"'challenge_id'" json:"challenge_id"`                       // The challenge id. The flag belongs to.
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`                   // The flag created time.
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at"`                   // The flag updated time.
}

func (f *Flag) TableName() string {
	return "flag"
}
