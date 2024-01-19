package response

type UserResponse struct {
	UserId    int64          `xorm:"'id'" json:"id"`
	Username  string         `xorm:"'username'" json:"username"`
	Name      string         `xorm:"'name'" json:"name"`
	Email     string         `xorm:"'email'" json:"email"`
	Role      int64          `xorm:"'role'" json:"role"`
	CreatedAt int64          `xorm:"'created_at'" json:"created_at"`
	UpdatedAt int64          `xorm:"'updated_at'" json:"updated_at"`
	Teams     []TeamResponse `xorm:"-" json:"teams,omitempty"`
}

type UserResponseWithTeamId struct {
	UserId    int64  `xorm:"'id'" json:"id"`
	Username  string `xorm:"'username'" json:"username"`
	Name      string `xorm:"'name'" json:"name"`
	Email     string `xorm:"'email'" json:"email"`
	Role      int64  `xorm:"'role'" json:"role"`
	CreatedAt int64  `xorm:"'created_at'" json:"created_at"`
	UpdatedAt int64  `xorm:"'updated_at'" json:"updated_at"`
	TeamId    int64  `xorm:"'team_id'" json:"team_id"`
}

type UserSimpleResponse struct {
	UserId   int64  `xorm:"'id'" json:"id"`
	Username string `xorm:"'username'" json:"username"`
	Name     string `xorm:"'name'" json:"name"`
}
