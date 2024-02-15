package model

import (
	"time"
)

type Submission struct {
	ID          uint       `json:"id"`                                               // The submission's id. As primary key.
	Flag        string     `gorm:"type:varchar(128);not null" json:"flag,omitempty"` // The flag which was submitted for judgement.
	Status      int        `gorm:"not null;default:0" json:"status"`                 // The status of the submission. (0-meaningless, 1-accepted, 2-incorrect, 3-cheat, 4-duplicate)
	UserID      uint       `gorm:"not null" json:"user_id"`                          // The user who submitted the flag.
	User        *User      `json:"user"`                                             // The user who submitted the flag.
	ChallengeID uint       `gorm:"not null" json:"challenge_id"`                     // The challenge which is related to this submission.
	Challenge   *Challenge `json:"challenge"`                                        // The challenge which is related to this submission.
	TeamID      uint       `json:"team_id,omitempty"`                                // The team which submitted the flag. (Must be set when GameID is set)
	Team        *Team      `json:"team,omitempty"`                                   // The team which submitted the flag.
	GameID      uint       `json:"game_id,omitempty"`                                // The game which is related to this submission. (Must be set when TeamID is set)
	Game        *Game      `json:"game,omitempty"`                                   // The game which is related to this submission.
	Pts         int64      `gorm:"default:0" json:"pts"`                             // The points of the submission.
	CreatedAt   time.Time  `gorm:"autoCreateTime" json:"created_at,omitempty"`       // The submission's creation time.
	UpdatedAt   time.Time  `gorm:"autoUpdateTime" json:"updated_at,omitempty"`       // The submission's last update time.
}
