package response

type FlagSimpleResponse struct {
	FlagID  int64  `xorm:"'id'" json:"id"`
	Content string `xorm:"'content'" json:"content"`
	Type    string `xorm:"'type'" json:"type"`
	Env     string `xorm:"'env'" json:"env"`
}
