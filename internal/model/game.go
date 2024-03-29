package model

import (
	"gorm.io/gorm"
)

type Game struct {
	ID                     uint    `json:"id"`                                                           // The game's id. As primary key.
	Title                  string  `gorm:"type:varchar(64);not null" json:"title,omitempty"`             // The game's title.
	Bio                    string  `gorm:"type:text" json:"bio,omitempty"`                               // The game's short description.
	Description            string  `gorm:"type:text" json:"description,omitempty"`                       // The game's description. (Markdown supported.)
	CoverURL               string  `gorm:"type:varchar(255)" json:"cover_url,omitempty"`                 // The game's cover image URL.
	PublicKey              string  `gorm:"type:varchar(255)" json:"public_key,omitempty"`                // The game's public key.
	PrivateKey             string  `gorm:"type:varchar(255)" json:"-"`                                   // The game's private key.
	IsEnabled              *bool   `gorm:"not null;default:false" json:"is_enabled,omitempty"`           // Whether the game is enabled.
	IsPublic               *bool   `gorm:"not null;default:true" json:"is_public,omitempty"`             // Whether the game is public.
	MemberLimitMin         int64   `gorm:"not null;default:1" json:"member_limit_min,omitempty"`         // The minimum team member limit.
	MemberLimitMax         int64   `gorm:"default:10" json:"member_limit_max,omitempty"`                 // The maximum team member limit.
	ParallelContainerLimit int64   `gorm:"not null;default:2" json:"parallel_container_limit,omitempty"` // The maximum parallel container limit.
	FirstBloodRewardRatio  float64 `gorm:"default:5" json:"first_blood_reward_ratio,omitempty"`          // The prize ratio of first blood.
	SecondBloodRewardRatio float64 `gorm:"default:3" json:"second_blood_reward_ratio,omitempty"`         // The prize ratio of second blood.
	ThirdBloodRewardRatio  float64 `gorm:"default:1" json:"third_blood_reward_ratio,omitempty"`          // The prize ratio of third blood.
	IsNeedWriteUp          *bool   `gorm:"not null;default:true" json:"is_need_write_up,omitempty"`      // Whether the game need write up.
	StartedAt              int64   `gorm:"not null" json:"started_at,omitempty"`                         // The game's start time. (Unix)
	EndedAt                int64   `gorm:"not null" json:"ended_at,omitempty"`                           // The game's end time. (Unix)
	CreatedAt              int64   `gorm:"autoUpdateTime:milli" json:"created_at,omitempty"`             // The game's creation time.
	UpdatedAt              int64   `gorm:"autoUpdateTime:milli" json:"updated_at,omitempty"`             // The game's last update time.
}

func (g *Game) BeforeDelete(db *gorm.DB) (err error) {
	db.Table("game_teams").Where("game_id = ?", g.ID).Delete(&GameTeam{})
	db.Table("game_challenges").Where("game_id = ?", g.ID).Delete(&GameChallenge{})
	return nil
}
