package response

type CategorySimpleResponse struct {
	CategoryID  int64  `xorm:"'id'" json:"id"`
	Name        string `xorm:"'name'" json:"name"`
	Description string `xorm:"'description'" json:"description"`
	ColorHex    string `xorm:"'color_hex'" json:"color_hex"`
	Mdi         string `xorm:"'mdi'" json:"mdi"`
}
