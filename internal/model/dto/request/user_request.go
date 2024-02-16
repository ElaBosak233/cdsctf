package request

type UserFindRequest struct {
	ID       uint     `json:"id"`
	Username string   `json:"username"`
	Name     string   `json:"name"`
	Email    string   `json:"email"`
	SortBy   []string `json:"sort_by"`
	Page     int      `json:"page"`
	Size     int      `json:"size"`
}

type UserBatchFindByTeamIdRequest struct {
	TeamID []uint
}

type UserRegisterRequest struct {
	Username string `binding:"required" json:"username"`
	Nickname string `binding:"required" json:"nickname"`
	Email    string `binding:"required" json:"email"`
	Password string `binding:"required" json:"password"`
}

type UserCreateRequest struct {
	Username string `binding:"required,max=20,min=3" json:"username" msg:"用户名必须位于 3 ~ 20 位"`
	Nickname string `binding:"required,min=2" json:"nickname" msg:"昵称必须有 2 位"`
	Email    string `binding:"required,email" json:"email" msg:"邮箱必须有效"`
	Password string `binding:"required,min=6" json:"password" msg:"密码必须大于 6 位"`
	GroupID  uint   `binding:"required,min=1,max=5" json:"group_id" msg:"权限值必须位于 1 ~ 5 位"`
}

type UserLoginRequest struct {
	Username string `binding:"required" json:"username"`
	Password string `binding:"required" json:"password"`
}

type UserUpdateRequest struct {
	ID       uint   `binding:"required" json:"id"`
	Nickname string `binding:"omitempty,min=2" json:"nickname" msg:"昵称必须有 2 位"`
	Username string `binding:"omitempty,max=20,min=3" json:"username,omitempty" msg:"用户名必须位于 3 ~ 20 位"`
	Password string `binding:"omitempty,min=6" json:"password,omitempty" msg:"密码必须大于 6 位"`
	Email    string `binding:"omitempty,email" json:"email,omitempty" msg:"邮箱必须有效"`
	GroupID  uint   `binding:"omitempty,min=1,max=5" json:"group_id,omitempty" msg:"权限值必须位于 1 ~ 5 位"`
}

type UserDeleteRequest struct {
	ID uint `binding:"required" json:"id"`
}
