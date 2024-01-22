package response

import "time"

type TeamResponse struct {
	TeamId    int64          `xorm:"'id'" json:"id"`
	TeamName  string         `xorm:"'name'" json:"name"`
	CaptainId int64          `xorm:"'captain_id'" json:"captain_id"`
	IsLocked  bool           `xorm:"'is_locked'" json:"is_locked"`
	CreatedAt time.Time      `xorm:"'created_at'" json:"created_at"`
	UpdatedAt time.Time      `xorm:"'updated_at'" json:"updated_at"`
	Users     []UserResponse `xorm:"-" json:"users,omitempty"`
	Captain   UserResponse   `xorm:"-" json:"captain,omitempty"`
}

type TeamResponseWithUserId struct {
	TeamId    int64     `xorm:"'id'" json:"id"`
	TeamName  string    `xorm:"'name'" json:"name"`
	CaptainId int64     `xorm:"'captain_id'" json:"captain_id"`
	IsLocked  bool      `xorm:"'is_locked'" json:"is_locked"`
	CreatedAt time.Time `xorm:"'created_at'" json:"created_at"`
	UpdatedAt time.Time `xorm:"'updated_at'" json:"updated_at"`
	UserId    int64     `xorm:"'user_id'" json:"user_id"`
}

type TeamSimpleResponse struct {
	TeamId    int64  `json:"id"`
	TeamName  string `json:"name"`
	CaptainId int64  `json:"captain_id"`
	IsLocked  bool   `json:"is_locked"`
}
