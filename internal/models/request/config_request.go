package request

type ConfigUpdateRequest struct {
	Title                  string `json:"title,omitempty"`
	Bio                    string `json:"bio,omitempty"`
	ParallelContainerLimit int64  `json:"parallel_container_limit,omitempty"`
	ContainerRequestLimit  int64  `json:"container_request_limit,omitempty"`
}
