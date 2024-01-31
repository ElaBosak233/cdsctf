package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	"xorm.io/xorm"
)

type SubmissionRepository interface {
	Insert(submission entity.Submission) (err error)
	Delete(id int64) (err error)
	Find(req request.SubmissionFindRequest) (submissions []response.SubmissionResponse, count int64, err error)
	BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error)
}

type SubmissionRepositoryImpl struct {
	Db *xorm.Engine
}

func NewSubmissionRepositoryImpl(Db *xorm.Engine) SubmissionRepository {
	return &SubmissionRepositoryImpl{Db: Db}
}

func (t *SubmissionRepositoryImpl) Insert(submission entity.Submission) (err error) {
	_, err = t.Db.Table("submissions").Insert(&submission)
	return err
}

func (t *SubmissionRepositoryImpl) Delete(id int64) (err error) {
	_, err = t.Db.Table("submissions").ID(id).Delete(&entity.Submission{})
	return err
}

func (t *SubmissionRepositoryImpl) Find(req request.SubmissionFindRequest) (submissions []response.SubmissionResponse, count int64, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.UserId != 0 && req.TeamId == 0 && req.GameId == 0 {
			q = q.Where("user_id = ?", req.UserId)
		}
		if req.ChallengeId != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeId)
		}
		if req.TeamId != -1 {
			q = q.Where("team_id = ?", req.TeamId)
		}
		if req.GameId != -1 {
			q = q.Where("game_id = ?", req.GameId)
		}
		if req.Status != 0 {
			q = q.Where("status = ?", req.Status)
		}
		if req.IsDetailed == 0 {
			q = q.Omit("flag")
		}
		return q
	}
	db := applyFilters(t.Db.Table("submissions"))
	ct := applyFilters(t.Db.Table("submissions"))
	count, err = ct.Count(&entity.Submission{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("submissions." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("submissions." + sortKey)
		}
	} else {
		db = db.Desc("submissions.id") // 默认采用 ID 降序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	db = db.Join("INNER", "users", "submissions.user_id = users.id").
		Join("INNER", "challenges", "submissions.challenge_id = challenges.id").
		Join("LEFT", "teams", "submissions.team_id = teams.id").
		Join("LEFT", "games", "submissions.game_id = games.id")
	err = db.Find(&submissions)
	return submissions, count, err
}

func (t *SubmissionRepositoryImpl) BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.UserId != 0 {
			q = q.Where("submissions.user_id = ?", req.UserId)
		}
		if req.TeamId != 0 {
			q = q.Where("submissions.team_id = ?", req.TeamId)
		}
		if req.GameId != -1 {
			q = q.Where("submissions.game_id = ?", req.GameId)
		}
		if req.Status != 0 {
			q = q.Where("submissions.status = ?", req.Status)
		}
		return q
	}
	db := applyFilters(t.Db.Table("submissions"))
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("submissions." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("submissions." + sortKey)
		}
	}
	db = db.Join("INNER", "users", "submissions.user_id = users.id").
		Join("LEFT", "teams", "submissions.team_id = teams.id").
		Join("LEFT", "challenges", "submissions.challenge_id = challenges.id").
		In("submissions.challenge_id", req.ChallengeId)
	_ = db.Find(&submissions)
	return submissions, err
}
