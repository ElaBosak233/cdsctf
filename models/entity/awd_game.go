package entity

// AwdGame is the awd configuration when a Game's type is "awd"
// This feature will not be implemented so fast in the near future.
type AwdGame struct {
	AwdGameID int64 `xorm:"id pk autoincr" json:"id"` // The awd game's id.
	GameID    int64 `xorm:"game_id" json:"game_id"`   // The game which is related to the awd game.
}
