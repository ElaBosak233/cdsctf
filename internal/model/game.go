package model

import "time"

type Game struct {
	ID                     int64      `xorm:"'id' pk autoincr" json:"id"`                                                    // The game's id. As primary key.
	Title                  string     `xorm:"'title' varchar(64) notnull" json:"title"`                                      // The game's title.
	Bio                    string     `xorm:"'bio' text" json:"bio"`                                                         // The game's short description.
	Description            string     `xorm:"'description' text" json:"description"`                                         // The game's description. (Markdown supported.)
	IsEnabled              *bool      `xorm:"'is_enabled' bool default(false) notnull" json:"is_enabled"`                    // Whether the game is enabled.
	IsPublic               *bool      `xorm:"'is_public' default(true) notnull" json:"is_public"`                            // Whether the game is public.
	Password               string     `xorm:"'password' varchar(255)" json:"password"`                                       // The game's password. Only enabled when the game is private.
	MemberLimitMin         int64      `xorm:"'member_limit_min' notnull default(1)" json:"member_limit_min"`                 // The minimum team member limit.
	MemberLimitMax         int64      `xorm:"'member_limit_max' default(10)" json:"member_limit_max"`                        // The maximum team member limit.
	ParallelContainerLimit int64      `xorm:"'parallel_container_limit' notnull default(2)" json:"parallel_container_limit"` // The maximum parallel container limit.
	FirstBloodRewardRatio  float64    `xorm:"'first_blood_reward_ratio' default(5)" json:"first_blood_reward_ratio"`         // The prize ratio of first blood.
	SecondBloodRewardRatio float64    `xorm:"'second_blood_reward_ratio' default(3)" json:"second_blood_reward_ratio"`       // The prize ratio of second blood.
	ThirdBloodRewardRatio  float64    `xorm:"'third_blood_reward_ratio' default(1)" json:"third_blood_reward_ratio"`         // The prize ratio of third blood.
	IsNeedWriteUp          *bool      `xorm:"'is_need_write_up' bool default(true) notnull" json:"is_need_write_up"`         // Whether the game need write up.
	StartedAt              int64      `xorm:"'started_at' notnull" json:"started_at"`                                        // The game's start time. (Unix)
	EndedAt                int64      `xorm:"'ended_at' notnull" json:"ended_at"`                                            // The game's end time. (Unix)
	CreatedAt              *time.Time `xorm:"'created_at' created" json:"created_at"`                                        // The game's creation time.
	UpdatedAt              *time.Time `xorm:"'updated_at' updated" json:"updated_at"`                                        // The game's last update time.
}

func (g *Game) TableName() string {
	return "game"
}
