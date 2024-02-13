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
	CategoryID    int64          `json:"category_id"`
	Duration      int64          `json:"duration"`
	Difficulty    int64          `json:"difficulty"`
	PracticePts   int64          `json:"practice_pts"`
	Images        *[]model.Image `json:"images"`
	Flags         *[]model.Flag  `json:"flags"`
}

type ChallengeUpdateRequest struct {
	ID            int64          `json:"id"`
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
	ChallengeId int64 `json:"id" binding:"required"`
}

type ChallengeFindRequest struct {
	IDs           []int64  `json:"id"`
	Category      string   `json:"category"`
	Title         string   `json:"title"`
	IsPracticable *bool    `json:"is_practicable"`
	IsDynamic     *bool    `json:"is_dynamic"`
	Difficulty    int64    `json:"difficulty"`
	UserID        int64    `json:"user_id"`
	GameID        int64    `json:"game_id"`
	TeamID        int64    `json:"team_id"`
	IsDetailed    *bool    `json:"is_detailed"`
	SortBy        []string `json:"sort_by"`
	Page          int      `json:"page"`
	Size          int      `json:"size"`
}
