package challenge

type CreateChallengeRequest struct {
	Title       string `validate:"required" json:"title"`
	Description string `json:"description"`
	UploaderId  string `validate:"required" json:"uploader_id"`
}
