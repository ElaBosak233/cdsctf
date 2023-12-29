package response

type InstanceStatusResponse struct {
	InstanceId string `json:"id"`
	Entry      string `json:"entry"`
	RemovedAt  int64  `json:"removed_at"`
	Status     string `json:"status"`
}

type InstanceResponse struct {
	InstanceId  string `json:"id"`
	Entry       string `json:"entry"`
	RemovedAt   int64  `json:"removed_at"`
	Status      string `json:"status"`
	ChallengeId string `json:"challenge_id"`
}
