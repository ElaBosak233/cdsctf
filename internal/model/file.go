package model

// File is only a struct for the file information.
// Not a real model for GORM.
type File struct {
	Name string `json:"name"`
	Size int64  `json:"size"`
}
