package response

type InstanceStatusResponse struct {
	InstanceId int64  `json:"id"`
	Entry      string `json:"entry"`
	RemovedAt  int64  `json:"removed_at"`
	Status     string `json:"status"`
}

type InstanceResponse struct {
	InstanceId  int64  `json:"id"`
	Entry       string `json:"entry"`
	RemovedAt   int64  `json:"removed_at"`
	Status      string `json:"status"`
	ChallengeId int64  `json:"challenge_id"`
}
