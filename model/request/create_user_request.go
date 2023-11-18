package request

type CreateUserRequest struct {
	Username string `validate:"required,min=3,max=20" json:"username"`
	Password string `validate:"required,min=6" json:"password"`
}
