package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	"xorm.io/xorm"
)

type ISubmissionRepository interface {
	Insert(submission model.Submission) (err error)
	Delete(id int64) (err error)
	Find(req request.SubmissionFindRequest) (submissions []response.SubmissionResponse, count int64, err error)
	BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error)
}

type SubmissionRepository struct {
	Db *xorm.Engine
}

func NewSubmissionRepository(Db *xorm.Engine) ISubmissionRepository {
	return &SubmissionRepository{Db: Db}
}

func (t *SubmissionRepository) Insert(submission model.Submission) (err error) {
	_, err = t.Db.Table("submission").Insert(&submission)
	return err
}

func (t *SubmissionRepository) Delete(id int64) (err error) {
	_, err = t.Db.Table("submission").ID(id).Delete(&model.Submission{})
	return err
}

func (t *SubmissionRepository) Find(req request.SubmissionFindRequest) (submissions []response.SubmissionResponse, count int64, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.UserID != 0 && req.TeamID == 0 && req.GameID == 0 {
			q = q.Where("user_id = ?", req.UserID)
		}
		if req.ChallengeID != 0 {
			q = q.Where("challenge_id = ?", req.ChallengeID)
		}
		if req.TeamID != -1 {
			q = q.Where("team_id = ?", req.TeamID)
		}
		if req.GameID != -1 {
			q = q.Where("game_id = ?", req.GameID)
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
	count, err = ct.Count(&model.Submission{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("submission." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("submission." + sortKey)
		}
	} else {
		db = db.Desc("submission.id") // 默认采用 IDs 降序排列
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

func (t *SubmissionRepository) BatchFind(req request.SubmissionBatchFindRequest) (submissions []response.SubmissionResponse, err error) {
	applyFilters := func(q *xorm.Session) *xorm.Session {
		if req.UserID != 0 {
			q = q.Where("submission.user_id = ?", req.UserID)
		}
		if req.TeamID != 0 {
			q = q.Where("submission.team_id = ?", req.TeamID)
		}
		if req.GameID != -1 {
			q = q.Where("submission.game_id = ?", req.GameID)
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
		In("submission.challenge_id", req.ChallengeID)
	_ = db.Find(&submissions)
	return submissions, err
}
