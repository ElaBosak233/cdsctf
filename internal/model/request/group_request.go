package request

type GroupFindRequest struct {
	ID   uint   `form:"id"`
	Name string `form:"name"`
}

type GroupUpdateRequest struct {
	ID          uint   `json:"id"`
	DisplayName string `json:"display_name"`
	Description string `json:"description"`
}
