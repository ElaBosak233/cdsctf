package implements

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/services"
	"math"
)

type SubmissionServiceImpl struct {
	SubmissionRepository repositorys.SubmissionRepository
}

func NewSubmissionServiceImpl(appRepository *repositorys.AppRepository) services.SubmissionService {
	return &SubmissionServiceImpl{
		SubmissionRepository: appRepository.SubmissionRepository,
	}
}

func (t *SubmissionServiceImpl) Create(req request.SubmissionCreateRequest) (err error) {
	err = t.SubmissionRepository.Insert(model.Submission{
		UserId:      req.UserId,
		ChallengeId: req.ChallengeId,
		TeamId:      req.TeamId,
		GameId:      req.GameId,
	})
	return err
}

func (t *SubmissionServiceImpl) Delete(id string) (err error) {
	err = t.SubmissionRepository.Delete(id)
	return err
}

func (t *SubmissionServiceImpl) Find(req request.SubmissionFindRequest) (submissions []model.Submission, pageCount int64, err error) {
	submissions, count, err := t.SubmissionRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return submissions, pageCount, err
}
