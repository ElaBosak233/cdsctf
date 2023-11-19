package account

type CreateGroupRequest struct {
	Name string `validate:"required,min=3,max=20" json:"name"`
}
