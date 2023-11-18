package request

type UpdateUserRequest struct {
	Id       string   `validate:"required"`
	Username string   `validate:"required,max=20,min=3" json:"username"`
	Password string   `validate:"required,min=6" json:"password"`
	GroupIds []string `json:"group_ids"`
}
