package request

type CategoryCreateRequest struct {
	Name        string `json:"name" binding:"required"`
	Description string `json:"description" binding:"required"`
	Color       string `json:"color" binding:"required"`
}

type CategoryFindRequest struct {
	ID   uint   `json:"id"`
	Name string `json:"name"`
}
