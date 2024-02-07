package request

import "github.com/elabosak233/pgshub/models/entity"

type ChallengeCreateRequest struct {
	Title         string          `json:"title"`
	Description   string          `json:"description"`
	HasAttachment *bool           `json:"has_attachment"`
	IsPracticable *bool           `json:"is_practicable"`
	IsDynamic     *bool           `json:"is_dynamic"`
	CategoryID    int64           `json:"category_id"`
	Duration      int64           `json:"duration"`
	Difficulty    int64           `json:"difficulty"`
	Images        *[]entity.Image `json:"images"`
	Flags         *[]entity.Flag  `json:"flags"`
}

type ChallengeUpdateRequest struct {
	ChallengeID   int64           `json:"id"`
	Title         string          `json:"title"`
	Description   string          `json:"description"`
	HasAttachment *bool           `json:"has_attachment"`
	IsPracticable *bool           `json:"is_practicable"`
	CategoryID    int64           `json:"category_id"`
	Duration      int64           `json:"duration"`
	Difficulty    int64           `json:"difficulty"`
	Images        *[]entity.Image `json:"images"`
	Flags         *[]entity.Flag  `json:"flags"`
}

type ChallengeDeleteRequest struct {
	ChallengeId int64 `json:"id" binding:"required"`
}

type ChallengeFindRequest struct {
	ChallengeIds  []int64  `json:"id"`
	Category      string   `json:"category"`
	Title         string   `json:"title"`
	IsPracticable *bool    `json:"is_practicable"`
	IsDynamic     *bool    `json:"is_dynamic"`
	Difficulty    int64    `json:"difficulty"`
	UserId        int64    `json:"user_id"`
	GameId        int64    `json:"game_id"`
	TeamId        int64    `json:"team_id"`
	IsDetailed    *bool    `json:"is_detailed"`
	SortBy        []string `json:"sort_by"`
	Page          int      `json:"page"`
	Size          int      `json:"size"`
}
