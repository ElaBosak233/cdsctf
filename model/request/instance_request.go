package request

type InstanceCreateRequest struct {
	ChallengeId string `binding:"required" json:"challenge_id"`
}

type InstanceRemoveRequest struct {
	InstanceId string `binding:"required" json:"id"`
}

type InstanceRenewRequest struct {
	InstanceId string `binding:"required" json:"id"`
}
