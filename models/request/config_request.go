package request

type Platform struct {
	Title string `json:"title,omitempty"`
	Bio   string `json:"bio,omitempty"`
}

type Container struct {
	ParallelLimit int64 `json:"parallel_limit,omitempty"`
	RequestLimit  int64 `json:"request_limit,omitempty"`
}

type User struct {
	AllowRegistration bool `json:"allow_registration,omitempty"`
}

type ConfigUpdateRequest struct {
	Platform  Platform  `json:"platform,omitempty"`
	Container Container `json:"container,omitempty"`
	User      User      `json:"user,omitempty"`
}
