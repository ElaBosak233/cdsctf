package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type GameChallengeResponse struct {
	*model.Challenge
	IsEnabled bool  `json:"is_enabled"`
	MaxPts    int   `json:"max_pts"`
	MinPts    int   `json:"min_pts"`
	Pts       int64 `json:"pts"`
}

type GameTeamResponse struct {
	*model.Team
	Solved    int    `json:"solved"`
	Rank      int    `json:"rank"`
	Pts       int64  `json:"pts"`
	IsAllowed bool   `json:"is_allowed"`
	Signature string `json:"signature"`
}
