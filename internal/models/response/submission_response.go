package response

import model "github.com/elabosak233/pgshub/internal/models/data"

type SubmissionResponse struct {
	model.Submission `xorm:"extends"`
	User             UserSimpleResponse `xorm:"extends" json:"user,omitempty"`
	Team             TeamSimpleResponse `json:"team,omitempty"`
}
