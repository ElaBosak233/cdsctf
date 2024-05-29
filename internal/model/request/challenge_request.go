package request

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type ChallengeCreateRequest struct {
	Title         string        `json:"title"`
	Description   string        `json:"description"`
	HasAttachment *bool         `json:"has_attachment"`
	AttachmentURL string        `json:"attachment_url"`
	IsPracticable *bool         `json:"is_practicable"`
	IsDynamic     *bool         `json:"is_dynamic"`
	CategoryID    uint          `json:"category_id"`
	Duration      int64         `json:"duration"`
	Difficulty    int64         `json:"difficulty"`
	PracticePts   int64         `json:"practice_pts"`
	ImageName     string        `json:"image_name"`
	CPULimit      *int64        `json:"cpu_limit"`
	MemoryLimit   *int64        `json:"memory_limit"`
	Ports         []*model.Port `json:"ports"`
	Envs          []*model.Env  `json:"envs"`
}

type ChallengeUpdateRequest struct {
	ID            uint          `json:"-"`
	Title         string        `json:"title"`
	Description   string        `json:"description"`
	HasAttachment *bool         `json:"has_attachment"`
	AttachmentURL string        `json:"attachment_url"`
	IsPracticable *bool         `json:"is_practicable"`
	IsDynamic     *bool         `json:"is_dynamic"`
	CategoryID    int64         `json:"category_id"`
	Duration      int64         `json:"duration"`
	Difficulty    int64         `json:"difficulty"`
	PracticePts   int64         `json:"practice_pts"`
	ImageName     string        `json:"image_name"`
	CPULimit      *int64        `json:"cpu_limit"`
	MemoryLimit   *int64        `json:"memory_limit"`
	Ports         []*model.Port `json:"ports"`
	Envs          []*model.Env  `json:"envs"`
}

type ChallengeDeleteRequest struct {
	ID uint `json:"-"`
}

type ChallengeFindRequest struct {
	ID            uint   `json:"id" form:"id"`
	CategoryID    *uint  `json:"category_id" form:"category_id"`
	Title         string `json:"title" form:"title"`
	IsPracticable *bool  `json:"is_practicable" form:"is_practicable"`
	IsDynamic     *bool  `json:"is_dynamic" form:"is_dynamic"`
	Difficulty    int64  `json:"difficulty" form:"difficulty"`
	UserID        uint   `json:"user_id" form:"user_id"`
	GameID        *uint  `json:"game_id" form:"game_id"`
	TeamID        *uint  `json:"team_id" form:"team_id"`
	IsDetailed    *bool  `json:"is_detailed" form:"is_detailed"`
	SubmissionQty int    `json:"submission_qty" form:"submission_qty"`
	Page          int    `json:"page" form:"page"`
	Size          int    `json:"size" form:"size"`
	SortKey       string `json:"sort_key" form:"sort_key"`
	SortOrder     string `json:"sort_order" form:"sort_order"`
}
