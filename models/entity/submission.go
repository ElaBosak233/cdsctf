package entity

import "time"

type Submission struct {
	SubmissionId int64     `xorm:"'id' index pk autoincr" json:"id"`                  // The submission's id. As primary key.
	Flag         string    `xorm:"'flag' varchar(128) notnull" json:"flag,omitempty"` // The flag which was submitted for judgement.
	Status       int       `xorm:"'status' notnull default(0)" json:"status"`         // The status of the submission. (0-meaningless, 1-accepted, 2-incorrect, 3-cheat, 4-duplicate)
	UserId       int64     `xorm:"'user_id' notnull index" json:"user_id"`            // The user who submitted the flag.
	ChallengeId  int64     `xorm:"'challenge_id' notnull" json:"challenge_id"`        // The challenge which is related to this submission.
	TeamId       int64     `xorm:"'team_id'" json:"team_id,omitempty"`                // The team which submitted the flag. (Must be set when GameId is set)
	GameId       int64     `xorm:"'game_id' index" json:"game_id,omitempty"`          // The game which is related to this submission. (Must be set when TeamId is set)
	Pts          int64     `xorm:"'pts' default(0)" json:"pts"`                       // The points of the submission.
	CreatedAt    time.Time `xorm:"'created_at' created" json:"created_at"`            // The submission's creation time.
	UpdatedAt    time.Time `xorm:"'updated_at' updated" json:"updated_at"`            // The submission's last update time.
}

func (s *Submission) TableName() string {
	return "submission"
}
