package database

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"go.uber.org/zap"
)

func initGroup() {
	var count int64
	db.Model(&model.Group{}).Count(&count)
	if count == 0 {
		zap.L().Warn("Groups do not exist, will be created soon.")
		defaultGroups := []model.Group{
			{
				Name:        "admin",
				DisplayName: "Administrator",
				Description: "The administrator has the highest authority.",
			},
			{
				Name:        "monitor",
				DisplayName: "Monitor",
				Description: "The monitor has the authority to control the games.",
			},
			{
				Name:        "user",
				DisplayName: "User",
				Description: "The user is the default role.",
			},
			{
				Name:        "banned",
				DisplayName: "Banned",
				Description: "The banned user has no authority.",
			},
		}
		err := db.Create(&defaultGroups).Error
		if err != nil {
			zap.L().Fatal("Groups initialization failed.", zap.Error(err))
			return
		}
		zap.L().Info("Groups created successfully.")
	}
}
