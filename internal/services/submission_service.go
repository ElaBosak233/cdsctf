package services

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
)

type SubmissionService interface {
	Create(req request.SubmissionCreateRequest) (err error)
	Delete(id string) (err error)
	Find(req request.SubmissionFindRequest) (submissions []model.Submission, pageCount int64, err error)
}
