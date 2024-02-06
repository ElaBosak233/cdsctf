package request

type ImageCreateRequest struct {
	Name        string              `json:"name"`
	CPULimit    float64             `json:"cpu_limit"`
	MemoryLimit int64               `json:"memory_limit"`
	Description string              `json:"description"`
	Ports       []PortCreateRequest `json:"ports"`
}
