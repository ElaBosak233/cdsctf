package response

type PortSimpleResponse struct {
	PortID      int64  `xorm:"'id'" json:"id"`
	Value       int    `xorm:"'value'" json:"value"`
	Description string `xorm:"'description'" json:"description"`
}
