package response

import "time"

type TeamResponse struct {
	ID          uint           `xorm:"'id'" json:"id"`
	Name        string         `xorm:"'name'" json:"name"`
	Description string         `xorm:"'description'" json:"description"`
	CaptainId   uint           `xorm:"'captain_id'" json:"captain_id"`
	IsLocked    bool           `xorm:"'is_locked'" json:"is_locked"`
	CreatedAt   time.Time      `xorm:"'created_at'" json:"created_at"`
	UpdatedAt   time.Time      `xorm:"'updated_at'" json:"updated_at"`
	Users       []UserResponse `xorm:"-" json:"users,omitempty"`
	Captain     UserResponse   `xorm:"-" json:"captain,omitempty"`
}

type TeamResponseWithUserId struct {
	ID          uint      `xorm:"'id'" json:"id"`
	Name        string    `xorm:"'name'" json:"name"`
	Description string    `xorm:"'description'" json:"description"`
	CaptainId   uint      `xorm:"'captain_id'" json:"captain_id"`
	IsLocked    bool      `xorm:"'is_locked'" json:"is_locked"`
	CreatedAt   time.Time `xorm:"'created_at'" json:"created_at"`
	UpdatedAt   time.Time `xorm:"'updated_at'" json:"updated_at"`
	UserId      uint      `xorm:"'user_id'" json:"user_id"`
}

type TeamSimpleResponse struct {
	ID          uint   `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	CaptainId   uint   `json:"captain_id"`
	IsLocked    bool   `json:"is_locked"`
}
