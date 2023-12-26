package controllers

import "github.com/gin-gonic/gin"

type SubmissionController interface {
	Find(ctx *gin.Context)
}
