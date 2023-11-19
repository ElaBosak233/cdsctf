package account

type UpdateGroupRequest struct {
	Id      string   `validate:"required"`
	Name    string   `validate:"required,max=20,min=3" json:"name"`
	UserIds []string `json:"user_ids"`
}
