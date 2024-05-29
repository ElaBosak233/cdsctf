package model

// Port is the mapping between the JeopardyImage and the exposed port of the image.
// Because of the port is only a subsidiary table, it doesn't need the creation time or updated time.
type Port struct {
	ID          uint       `json:"id"`                                  // The port's id. As primary key.
	ChallengeID uint       `gorm:"not null;" json:"challenge_id"`       // The JeopardyImage which the port belongs to.
	Challenge   *Challenge `json:"challenge,omitempty"`                 // The JeopardyImage which the port belongs to.
	Value       int        `gorm:"not null;" json:"value"`              // The port number.
	Description string     `gorm:"type:varchar(32)" json:"description"` // The port's description.
}
