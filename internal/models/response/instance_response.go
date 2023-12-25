package response

type InstanceStatusResponse struct {
	InstanceId string `json:"id"`
	Entry      string `json:"entry"`
	RemoveAt   int64  `json:"remove_at"`
	Status     string `json:"status"`
}

type InstanceResponse struct {
	InstanceId  string `json:"id"`
	Entry       string `json:"entry"`
	RemoveAt    int64  `json:"remove_at"`
	Status      string `json:"status"`
	ChallengeId string `json:"challenge_id"`
}
