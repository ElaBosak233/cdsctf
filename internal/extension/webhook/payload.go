package webhook

import "github.com/elabosak233/cloudsdale/internal/model"

type Payload struct {
	GameID uint        `json:"game_id,omitempty"`
	Game   *model.Game `json:"game,omitempty"`
}
