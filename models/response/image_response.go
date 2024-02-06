package response

type ImageResponse struct {
	ImageSimpleResponse
	Ports []PortSimpleResponse `xorm:"-" json:"ports"`
}

type ImageSimpleResponse struct {
	ImageID     int64   `xorm:"'id'" json:"id"`
	Name        string  `xorm:"'name'" json:"name"`
	Description string  `xorm:"'description'" json:"description"`
	MemoryLimit int64   `xorm:"'memory_limit'" json:"memory_limit"`
	CPULimit    float64 `xorm:"'cpu_limit'" json:"cpu_limit"`
}
