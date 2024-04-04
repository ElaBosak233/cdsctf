package request

type UserFindRequest struct {
	ID        uint   `json:"id" form:"id"`
	Username  string `json:"username" form:"username"`
	Name      string `json:"name" form:"name"`
	Email     string `json:"email" form:"email"`
	Page      int    `json:"page" form:"page"`
	Size      int    `json:"size" form:"size"`
	SortKey   string `json:"sort_key" form:"sort_key"`
	SortOrder string `json:"sort_order" form:"sort_order"`
}

type UserRegisterRequest struct {
	Username     string `binding:"required" json:"username"`
	Nickname     string `binding:"required" json:"nickname"`
	Email        string `binding:"required" json:"email"`
	Password     string `binding:"required" json:"password"`
	CaptchaToken string `json:"token"`
	RemoteIP     string `json:"-"`
}

type UserCreateRequest struct {
	Username string `binding:"required,max=20,min=3,ascii" json:"username"`
	Nickname string `binding:"required,min=2" json:"nickname"`
	Email    string `binding:"required,email" json:"email"`
	Password string `binding:"required,min=6" json:"password"`
	GroupID  uint   `binding:"required,min=1,max=5" json:"group_id"`
}

type UserLoginRequest struct {
	Username string `binding:"required,ascii" json:"username"`
	Password string `binding:"required" json:"password"`
}

type UserUpdateRequest struct {
	ID       uint   `json:"-"`
	Nickname string `binding:"omitempty,min=2" json:"nickname"`
	Username string `binding:"omitempty,max=20,min=3" json:"username,omitempty"`
	Password string `binding:"omitempty,min=6" json:"password,omitempty"`
	Email    string `binding:"omitempty,email" json:"email,omitempty"`
	GroupID  uint   `binding:"omitempty,min=1,max=5" json:"group_id,omitempty"`
}

type UserDeleteRequest struct {
	ID uint `json:"-"`
}
