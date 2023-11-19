package container

type CreateContainerRequest struct {
	UserId      string `validate:"required" json:"user_id"`
	ChallengeId string `validate:"required, email" json:"email"`
}
