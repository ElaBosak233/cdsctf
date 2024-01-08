package response

import model "github.com/elabosak233/pgshub/internal/models/data"

type SubmissionResponse struct {
	model.Submission
	User UserSimpleResponse `json:"user,omitempty"`
	Team TeamSimpleResponse `json:"team,omitempty"`
}
