package model

type Pod struct {
	ID          uint       `json:"id"`
	GameID      *uint      `gorm:"index;null;default:null" json:"game_id"`
	Game        *Game      `gorm:"foreignkey:GameID;association_foreignkey:ID" json:"game,omitempty"`
	UserID      *uint      `gorm:"index;null;default:null"  json:"user_id"`
	User        *User      `gorm:"foreignkey:UserID;association_foreignkey:ID" json:"user,omitempty"`
	TeamID      *uint      `gorm:"index;null;default:null" json:"team_id"`
	Team        *Team      `gorm:"foreignkey:TeamID;association_foreignkey:ID" json:"team,omitempty"`
	ChallengeID *uint      `gorm:"index;null;default:null" json:"challenge_id"`
	Challenge   *Challenge `gorm:"foreignkey:ChallengeID;association_foreignkey:ID" json:"challenge,omitempty"`
	RemovedAt   int64      `json:"removed_at"`
	Nats        []*Nat     `json:"nats,omitempty"`
}
