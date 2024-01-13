package data

type Team struct {
	TeamId    int64  `xorm:"'id' pk autoincr" json:"id"`
	TeamName  string `xorm:"varchar(64) 'name' notnull" json:"name"`              // 团队名
	CaptainId int64  `xorm:"'captain_id' varchar(36) notnull" json:"captain_id"`  // 队长用户 Id
	IsLocked  bool   `xorm:"'is_locked' notnull default(false)" json:"is_locked"` // 是否锁定
	CreatedAt int64  `xorm:"created 'created_at'" json:"created_at"`              // 创建时间
	UpdatedAt int64  `xorm:"updated 'updated_at'" json:"updated_at"`              // 更新时间
}
