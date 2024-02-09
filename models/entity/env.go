package entity

type Env struct {
	ID      int64  `xorm:"'id' pk autoincr" json:"id"`
	Key     string `xorm:"'key' varchar(128) notnull" json:"key"`
	Value   string `xorm:"'value' varchar(128) notnull" json:"value"`
	ImageID int64  `xorm:"'image_id' notnull" json:"image_id"`
}

func (e *Env) TableName() string {
	return "env"
}
