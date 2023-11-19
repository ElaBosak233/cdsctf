package account

type CreateUserRequest struct {
	Username string `validate:"required,min=3,max=20" json:"username"`
	Email    string `validate:"required,email" json:"email"`
	Password string `validate:"required,min=6" json:"password"`
}
