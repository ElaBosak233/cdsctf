package database

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"go.uber.org/zap"
)

func initCategory() {
	var count int64
	db.Model(&model.Category{}).Count(&count)
	if count == 0 {
		zap.L().Warn("Categories do not exist, will be created soon.")
		defaultCategories := []model.Category{
			{
				Name:        "misc",
				Description: "misc",
				Color:       "#3F51B5",
				Icon:        "fingerprint",
			},
			{
				Name:        "web",
				Description: "web",
				Color:       "#009688",
				Icon:        "language",
			},
			{
				Name:        "pwn",
				Description: "pwn",
				Color:       "#673AB7",
				Icon:        "function",
			},
			{
				Name:        "crypto",
				Description: "crypto",
				Color:       "#607D8B",
				Icon:        "tag",
			},
			{
				Name:        "reverse",
				Description: "reverse",
				Color:       "#6D4C41",
				Icon:        "keyboard_double_arrow_left",
			},
		}
		err := db.Create(&defaultCategories).Error
		if err != nil {
			zap.L().Fatal("Category initialization failed.", zap.Error(err))
			return
		}
	}
}
