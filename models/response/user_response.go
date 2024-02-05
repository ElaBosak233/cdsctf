package response

import "time"

type UserResponse struct {
	UserID    int64                `xorm:"'id'" json:"id"`
	Username  string               `xorm:"'username'" json:"username"`
	Nickname  string               `xorm:"'nickname'" json:"nickname"`
	Email     string               `xorm:"'email'" json:"email"`
	Role      int64                `xorm:"'role'" json:"role"`
	CreatedAt time.Time            `xorm:"'created_at'" json:"created_at"`
	UpdatedAt time.Time            `xorm:"'updated_at'" json:"updated_at"`
	Teams     []TeamSimpleResponse `xorm:"-" json:"teams,omitempty"`
}

type UserResponseWithTeamId struct {
	UserID    int64     `xorm:"'id'" json:"id"`
	Username  string    `xorm:"'username'" json:"username"`
	Nickname  string    `xorm:"'nickname'" json:"nickname"`
	Email     string    `xorm:"'email'" json:"email"`
	Role      int64     `xorm:"'role'" json:"role"`
	CreatedAt time.Time `xorm:"'created_at'" json:"created_at"`
	UpdatedAt time.Time `xorm:"'updated_at'" json:"updated_at"`
	TeamId    int64     `xorm:"'team_id'" json:"team_id"`
}

type UserSimpleResponse struct {
	UserID   int64  `xorm:"'id'" json:"id"`
	Username string `xorm:"'username'" json:"username"`
	Nickname string `xorm:"'nickname'" json:"nickname"`
}
