package model

// Flag is the answer of a Challenge.
// Because of the Flag is only a subsidiary table, it doesn't need the creation time or updated time.
type Flag struct {
	ID          uint       `json:"id"`                                                      // The flag id.
	Type        string     `gorm:"type:varchar(16);not null;default:'static';" json:"type"` // The flag type. ("static"/"dynamic"/"pattern")
	Banned      *bool      `gorm:"not null;default:false;" json:"banned"`                   // Whether the flag is banned. If banned, the user who submitted the flag will be judged as cheating.
	Value       string     `gorm:"type:varchar(255);" json:"value"`                         // The flag content. Maybe a string or a regex, or the placeholder for dynamic challenges. (Such as "flag{friendsh1p_1s_magic}" or "flag{[a-zA-Z]{5}}" or "flag{[UUID]}")
	Env         string     `gorm:"type:varchar(16);" json:"env"`                            // The environment variable which is used to be injected with the flag.
	ChallengeID uint       `json:"challenge_id"`                                            // The challenge id. The flag belongs to.
	Challenge   *Challenge `json:"challenge"`                                               // The challenge which the flag belongs to.
}
