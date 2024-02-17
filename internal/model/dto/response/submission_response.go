package response

import (
	model "github.com/elabosak233/cloudsdale/internal/model"
)

type SubmissionResponse struct {
	model.Submission `xorm:"extends"`
	User             UserSimpleResponse      `xorm:"extends" json:"user,omitempty"`
	Challenge        ChallengeSimpleResponse `xorm:"extends" json:"challenge,omitempty"`
	Team             TeamSimpleResponse      `xorm:"extends" json:"team,omitempty"`
	Game             GameSimpleResponse      `xorm:"extends" json:"game,omitempty"`
}
