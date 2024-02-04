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
	_, err = t.Db.Table("submission").Insert(&submission)
	return err
}

func (t *SubmissionRepositoryImpl) Delete(id int64) (err error) {
	_, err = t.Db.Table("submission").ID(id).Delete(&entity.Submission{})
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
	db := applyFilters(t.Db.Table("submission"))
	ct := applyFilters(t.Db.Table("submission"))
	count, err = ct.Count(&entity.Submission{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("submission." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("submission." + sortKey)
		}
	} else {
		db = db.Desc("submission.id") // 默认采用 ID 降序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	db = db.Join("INNER", "account", "submission.user_id = account.id").
		Join("INNER", "challenge", "submission.challenge_id = challenge.id").
		Join("LEFT", "team", "submission.team_id = team.id").
		Join("LEFT", "game", "submission.game_id = game.id")
	err = db.Find(&submissions)
	return submissions, count, err
}

func (t *SubmissionRepositoryImpl) BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.UserId != 0 {
			q = q.Where("submission.user_id = ?", req.UserId)
		}
		if req.TeamId != 0 {
			q = q.Where("submission.team_id = ?", req.TeamId)
		}
		if req.GameId != -1 {
			q = q.Where("submission.game_id = ?", req.GameId)
		}
		if req.Status != 0 {
			q = q.Where("submission.status = ?", req.Status)
		}
		return q
	}
	db := applyFilters(t.Db.Table("submission"))
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("submission." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("submission." + sortKey)
		}
	}
	db = db.Join("INNER", "account", "submission.user_id = account.id").
		Join("LEFT", "team", "submission.team_id = team.id").
		Join("LEFT", "challenge", "submission.challenge_id = challenge.id").
		In("submission.challenge_id", req.ChallengeId)
	_ = db.Find(&submissions)
	return submissions, err
}
