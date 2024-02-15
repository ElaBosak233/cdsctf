package request

import (
	"github.com/elabosak233/pgshub/internal/model"
)

type ChallengeCreateRequest struct {
	Title         string         `json:"title"`
	Description   string         `json:"description"`
	HasAttachment *bool          `json:"has_attachment"`
	IsPracticable *bool          `json:"is_practicable"`
	IsDynamic     *bool          `json:"is_dynamic"`
	CategoryID    uint           `json:"category_id"`
	Duration      int64          `json:"duration"`
	Difficulty    int64          `json:"difficulty"`
	PracticePts   int64          `json:"practice_pts"`
	Images        *[]model.Image `json:"images"`
	Flags         *[]model.Flag  `json:"flags"`
}

type ChallengeUpdateRequest struct {
	ID            uint           `json:"id"`
	Title         string         `json:"title"`
	Description   string         `json:"description"`
	HasAttachment *bool          `json:"has_attachment"`
	IsPracticable *bool          `json:"is_practicable"`
	CategoryID    int64          `json:"category_id"`
	Duration      int64          `json:"duration"`
	Difficulty    int64          `json:"difficulty"`
	PracticePts   int64          `json:"practice_pts"`
	Images        *[]model.Image `json:"images"`
	Flags         *[]model.Flag  `json:"flags"`
}

type ChallengeDeleteRequest struct {
	ID uint `json:"id" binding:"required"`
}

type ChallengeFindRequest struct {
	IDs           []uint   `json:"id"`
	Category      string   `json:"category"`
	Title         string   `json:"title"`
	IsPracticable *bool    `json:"is_practicable"`
	IsDynamic     *bool    `json:"is_dynamic"`
	Difficulty    int64    `json:"difficulty"`
	UserID        uint     `json:"user_id"`
	GameID        *uint    `json:"game_id"`
	TeamID        *uint    `json:"team_id"`
	IsDetailed    *bool    `json:"is_detailed"`
	SortBy        []string `json:"sort_by"`
	Page          int      `json:"page"`
	Size          int      `json:"size"`
}
