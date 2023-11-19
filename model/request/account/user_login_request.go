package account

type UserLoginRequest struct {
	Id       string `validate:"required" json:"id"`
	Password string `validate:"required" json:"password"`
}
