package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type ChallengeResponse struct {
	ID            uint                `json:"id"`
	Title         string              `gorm:"type:varchar(32);not null;" json:"title"`
	Description   string              `gorm:"type:text;not null;" json:"description"`
	CategoryID    uint                `gorm:"not null;" json:"category_id"`
	Category      *model.Category     `json:"category,omitempty"`
	HasAttachment *bool               `gorm:"not null;default:false" json:"has_attachment"`
	AttachmentURL string              `gorm:"type:varchar(255);" json:"attachment_url"`
	IsPracticable *bool               `gorm:"not null;default:false" json:"is_practicable"`
	IsDynamic     *bool               `gorm:"default:false" json:"is_dynamic"`
	Difficulty    int64               `gorm:"default:1" json:"difficulty"`
	PracticePts   int64               `gorm:"default:200" json:"practice_pts"`
	Duration      int64               `gorm:"default:1800" json:"duration,omitempty"`
	CreatedAt     int64               `json:"created_at"`
	UpdatedAt     int64               `json:"updated_at"`
	Flags         []*model.Flag       `json:"flags,omitempty"`
	Hints         []*model.Hint       `json:"hints,omitempty"`
	Images        []*model.Image      `json:"images,omitempty"`
	Submissions   []*model.Submission `json:"submissions,omitempty"`
	Solved        *model.Submission   `json:"solved"`
}

type ChallengeSimpleResponse struct {
	ID          int64  `xorm:"'id'" json:"id"`
	Title       string `xorm:"'title'" json:"title"`
	Description string `xorm:"'description'" json:"description"`
	Category    string `xorm:"'category'" json:"category"`
}
