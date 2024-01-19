package response

import model "github.com/elabosak233/pgshub/models/entity"

type SubmissionResponse struct {
	model.Submission `xorm:"extends"`
	User             UserSimpleResponse      `xorm:"extends" json:"user,omitempty"`
	Challenge        ChallengeSimpleResponse `xorm:"extends" json:"challenge,omitempty"`
	Team             TeamSimpleResponse      `json:"team,omitempty"`
}
