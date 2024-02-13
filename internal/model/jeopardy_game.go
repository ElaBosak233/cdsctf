package model

// JeopardyGame is the jeopardy configuration when the Game's type is "jeopardy"
type JeopardyGame struct {
	ID                     int64   `xorm:"'id' pk autoincr" json:"id"`                                                    // The jeopardy game's id.
	GameID                 int64   `xorm:"'game_id'" json:"game_id"`                                                      // The game which is related to this jeopardy game.
	ParallelContainerLimit int64   `xorm:"'parallel_container_limit' notnull default(2)" json:"parallel_container_limit"` // The maximum parallel container limit.
	FirstBloodRewardRatio  float64 `xorm:"'first_blood_reward_ratio' default(5)" json:"first_blood_reward_ratio"`         // The prize ratio of first blood.
	SecondBloodRewardRatio float64 `xorm:"'second_blood_reward_ratio' default(3)" json:"second_blood_reward_ratio"`       // The prize ratio of second blood.
	ThirdBloodRewardRatio  float64 `xorm:"'third_blood_reward_ratio' default(1)" json:"third_blood_reward_ratio"`         // The prize ratio of third blood.
	IsNeedWriteUp          *bool   `xorm:"'is_need_write_up' bool default(true) notnull" json:"is_need_write_up"`         // Whether the game need write up.
}
