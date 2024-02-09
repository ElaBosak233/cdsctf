package entity

// FlagGen is the generated flag which is injected into the container.
// It will be generated when Flag's type is "dynamic".
type FlagGen struct {
	ID    int64  `xorm:"'id' pk autoincr" json:"id"`
	Flag  string `xorm:"'flag' varchar(128)" json:"flag"`
	PodID int64  `xorm:"'pod_id' notnull" json:"pod_id"`
}

func (f *FlagGen) TableName() string {
	return "flag_gen"
}
