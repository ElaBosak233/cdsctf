package entity

// Game 比赛对象
type Game struct {
	GameId                 int64   `xorm:"'id' pk autoincr" json:"id"`
	Title                  string  `xorm:"'title' varchar(64) notnull" json:"title"`                                      // 比赛标题
	Bio                    string  `xorm:"'bio' text" json:"bio"`                                                         // 比赛简介
	Description            string  `xorm:"'description' text" json:"description"`                                         // 比赛描述
	MemberLimitMin         int64   `xorm:"'member_limit_min' notnull default(1)" json:"member_limit_min"`                 // 团队人数最小值
	MemberLimitMax         int64   `xorm:"'member_limit_max' default(10)" json:"member_limit_max"`                        // 团队人数最大值
	ParallelContainerLimit int64   `xorm:"'parallel_container_limit' notnull default(2)" json:"parallel_container_limit"` // 容器并行限制
	FirstBloodRewardRatio  float64 `xorm:"'first_blood_reward_ratio' default(5)" json:"first_blood_reward_ratio"`         // 一血奖励比例
	SecondBloodRewardRatio float64 `xorm:"'second_blood_reward_ratio' default(3)" json:"second_blood_reward_ratio"`       // 二血奖励比例
	ThirdBloodRewardRatio  float64 `xorm:"'third_blood_reward_ratio' default(1)" json:"third_blood_reward_ratio"`         // 三血奖励比例
	IsNeedWriteUp          bool    `xorm:"'is_need_write_up' bool default(1) notnull" json:"is_need_write_up"`            // 是否需要提交 WP
	StartedAt              int64   `xorm:"'started_at' notnull" json:"started_at"`                                        // 比赛开始时间
	EndedAt                int64   `xorm:"'ended_at' notnull" json:"ended_at"`                                            // 比赛结束时间
	CreatedAt              int64   `xorm:"'created_at' created" json:"created_at"`                                        // 创建时间
	UpdatedAt              int64   `xorm:"'updated_at' updated" json:"updated_at"`                                        // 更新时间
}
