package request

type PortCreateRequest struct {
	Value       int    `xorm:"'value' notnull" json:"value"`
	Description string `xorm:"'description' varchar(32)" json:"description"`
}
