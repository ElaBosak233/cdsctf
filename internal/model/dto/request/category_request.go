package request

type CategoryCreateRequest struct {
	Name        string `json:"name" binding:"required"`
	Description string `json:"description" binding:"required"`
	Color       string `json:"color" binding:"required"`
	Icon        string `json:"icon" binding:"required"`
}

type CategoryUpdateRequest struct {
	ID          uint   `json:"id" binding:"required"`
	Name        string `json:"name" binding:"required"`
	Description string `json:"description" binding:"required"`
	Color       string `json:"color" binding:"required"`
	Icon        string `json:"icon" binding:"required"`
}

type CategoryFindRequest struct {
	ID   uint   `json:"id"`
	Name string `json:"name"`
}
