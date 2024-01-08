package request

type UserFindRequest struct {
	UserId   string `json:"id"`
	Username string `json:"username"`
	Name     string `json:"name"`
	Email    string `json:"email"`
	Role     int64  `json:"role"`
	Page     int    `json:"page"`
	Size     int    `json:"size"`
}

type UserRegisterRequest struct {
	Username string `binding:"required" json:"username"`
	Name     string `binding:"required" json:"name"`
	Email    string `binding:"required" json:"email"`
	Password string `binding:"required" json:"password"`
}

type UserCreateRequest struct {
	Username string `binding:"required" json:"username"`
	Name     string `binding:"required" json:"name"`
	Email    string `binding:"required" json:"email"`
	Password string `binding:"required" json:"password"`
	Role     int64  `binding:"required" json:"role"`
}

type UserLoginRequest struct {
	Username string `binding:"required" json:"username"`
	Password string `binding:"required" json:"password"`
}

type UserUpdateRequest struct {
	UserId   string `binding:"required" json:"id"`
	Username string `binding:"max=20,min=3" json:"username"`
	Password string `binding:"min=6" json:"password"`
	Email    string `binding:"email" json:"email"`
	Role     int64  `json:"role"`
}

type UserDeleteRequest struct {
	UserId string `binding:"required" json:"id"`
}
