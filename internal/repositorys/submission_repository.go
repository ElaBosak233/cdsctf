package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
)

type SubmissionRepository interface {
	Insert(submission model.Submission) (err error)
	Delete(id string) (err error)
	Find(req request.SubmissionFindRequest) (submissions []model.Submission, count int64, err error)
}
