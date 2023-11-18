package request

type AddUserToGroupRequest struct {
	Id string `validate:"required"`
}
