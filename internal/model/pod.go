package model

import "gorm.io/gorm"

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
	Flag        string     `json:"flag,omitempty"` // The generated flag, which will be injected into the container.
	RemovedAt   int64      `json:"removed_at"`
	CreatedAt   int64      `gorm:"autoUpdateTime:milli" json:"created_at,omitempty"`
	Nats        []*Nat     `json:"nats,omitempty"`
}

func (p *Pod) Simplify() {
	p.Flag = ""
	if p.Challenge != nil {
		p.Challenge.Simplify()
	}
}

func (p *Pod) BeforeDelete(db *gorm.DB) (err error) {
	db.Table("nats").Where("pod_id = ?", p.ID).Delete(&Nat{})
	return nil
}
