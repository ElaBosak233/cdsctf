package model

type Webhook struct {
	ID     uint   `json:"id"`
	URL    string `gorm:"type:varchar(255);" json:"url"`            // The payload URL of the webhook.
	Type   string `gorm:"type:varchar(64);" json:"type"`            // The type of the webhook. Such as "application/json" or "application/x-www-form-urlencoded".
	Secret string `gorm:"type:varchar(255);" json:"secret"`         // The secret of the webhook.
	SSL    *bool  `gorm:"default:false;" json:"ssl"`                // The SSL verification of the webhook.
	GameID *uint  `gorm:"index;not null;" json:"game_id,omitempty"` // The game which this webhook belongs to.
	Game   *Game  `json:"game,omitempty"`                           // The game which this webhook belongs to.
}
