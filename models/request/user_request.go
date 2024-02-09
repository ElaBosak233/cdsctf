package request

type UserFindRequest struct {
	ID       int64    `json:"id"`
	Username string   `json:"username"`
	Name     string   `json:"name"`
	Email    string   `json:"email"`
	Role     int64    `json:"role"`
	SortBy   []string `json:"sort_by"`
	Page     int      `json:"page"`
	Size     int      `json:"size"`
}

type UserBatchFindByTeamIdRequest struct {
	TeamID []int64
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
	Role     int64  `binding:"required,min=1,max=5" json:"role" msg:"权限值必须位于 1 ~ 5 位"`
}

type UserLoginRequest struct {
	Username string `binding:"required" json:"username"`
	Password string `binding:"required" json:"password"`
}

type UserUpdateRequest struct {
	ID       int64  `binding:"required" json:"id"`
	Nickname string `binding:"omitempty,min=2" json:"nickname" msg:"昵称必须有 2 位"`
	Username string `binding:"omitempty,max=20,min=3" json:"username,omitempty" msg:"用户名必须位于 3 ~ 20 位"`
	Password string `binding:"omitempty,min=6" json:"password,omitempty" msg:"密码必须大于 6 位"`
	Email    string `binding:"omitempty,email" json:"email,omitempty" msg:"邮箱必须有效"`
	Role     int64  `binding:"omitempty,min=1,max=5" json:"role,omitempty" msg:"权限值必须位于 1 ~ 5 位"`
}

type UserDeleteRequest struct {
	ID int64 `binding:"required" json:"id"`
}
