package account

type AddUserToGroupRequest struct {
	UserId  string `validate:"required" json:"user_id"`
	GroupId string `validate:"required" json:"group_id"`
}
