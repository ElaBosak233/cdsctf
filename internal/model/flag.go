package model

// Flag is the answer of a Challenge.
// Because of the Flag is only a subsidiary table, it doesn't need the creation time or updated time.
type Flag struct {
	ID          int64  `xorm:"'id' pk autoincr" json:"id"`                               // The flag id.
	Type        string `xorm:"'type' varchar(16) notnull default('static')" json:"type"` // The flag type. ("static"/"dynamic"/"pattern")
	Value       string `xorm:"'value' varchar(255)" json:"value"`                        // The flag content. Maybe a string or a regex, or the placeholder for dynamic challenges. (Such as "flag{friendsh1p_1s_magic}" or "flag{[a-zA-Z]{5}}" or "flag{[UUID]}")
	Env         string `xorm:"'env' varchar(16)" json:"env"`                             // The environment variable which is used to be injected with the flag.
	ChallengeID int64  `xorm:"'challenge_id'" json:"challenge_id"`                       // The challenge id. The flag belongs to.
}

func (f *Flag) TableName() string {
	return "flag"
}
