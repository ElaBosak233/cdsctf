package model

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"gorm.io/gorm"
	"os"
	"path"
)

// Challenge is the challenge for Jeopardy-style CTF game.
type Challenge struct {
	ID            uint          `json:"id"`                                                     // The challenge's id. As primary key.
	Title         string        `gorm:"type:varchar(32);not null;" json:"title"`                // The challenge's title.
	Description   string        `gorm:"type:text;not null;" json:"description"`                 // The challenge's description.
	CategoryID    uint          `gorm:"not null;" json:"category_id"`                           // The challenge's category.
	Category      *Category     `json:"category,omitempty"`                                     // The challenge's category.
	Attachment    *File         `gorm:"-" json:"attachment"`                                    // The challenge's attachment.
	IsPracticable *bool         `gorm:"not null;default:false" json:"is_practicable,omitempty"` // Whether the challenge is practicable. (Is the practice field visible.)
	IsDynamic     *bool         `gorm:"default:false" json:"is_dynamic"`                        // Whether the challenge is based on dynamic container.
	Difficulty    int64         `gorm:"default:1" json:"difficulty"`                            // The degree of difficulty. (From 1 to 5)
	PracticePts   int64         `gorm:"default:200" json:"practice_pts,omitempty"`              // The points will be given when the challenge is solved in practice field.
	Duration      int64         `gorm:"default:1800" json:"duration,omitempty"`                 // The duration of container maintenance in the initial state. (Seconds)
	ImageName     string        `gorm:"type:varchar(255);" json:"image_name,omitempty"`         // The challenge's image name.
	CPULimit      int64         `gorm:"default:1" json:"cpu_limit,omitempty"`                   // The challenge's CPU limit. (0 means no limit)
	MemoryLimit   int64         `gorm:"default:64" json:"memory_limit,omitempty"`               // The challenge's memory limit. (0 means no limit)
	Flags         []*Flag       `json:"flags,omitempty"`
	Ports         []*Port       `json:"ports,omitempty"`
	Envs          []*Env        `json:"envs,omitempty"`
	Solved        *Submission   `json:"solved,omitempty"`
	SolvedTimes   int           `gorm:"-" json:"solved_times"`
	Submissions   []*Submission `json:"-"`
	Bloods        []*Submission `gorm:"-" json:"bloods,omitempty"`
	CreatedAt     int64         `gorm:"autoUpdateTime:milli" json:"created_at,omitempty"` // The challenge's creation time.
	UpdatedAt     int64         `gorm:"autoUpdateTime:milli" json:"updated_at,omitempty"` // The challenge's last update time.
}

func (c *Challenge) Simplify() {
	c.ImageName = ""
	c.CPULimit = 0
	c.MemoryLimit = 0
	c.Flags = nil
	c.Ports = nil
	c.Envs = nil
}

func (c *Challenge) AfterFind(db *gorm.DB) (err error) {
	p := path.Join(utils.MediaPath, "challenges", fmt.Sprintf("%d", c.ID))
	var name string
	var size int64
	if files, _err := os.ReadDir(p); _err == nil {
		for _, file := range files {
			name = file.Name()
			info, _ := file.Info()
			size = info.Size()
			break
		}
	}
	attachment := File{
		Name: name,
		Size: size,
	}
	c.Attachment = &attachment
	return nil
}

func (c *Challenge) BeforeUpdate(db *gorm.DB) (err error) {
	if c.Ports != nil {
		db.Table("ports").Where("challenge_id = ?", c.ID).Delete(&Port{})
		db.Table("envs").Where("challenge_id = ?", c.ID).Delete(&Env{})
	}
	return nil
}

func (c *Challenge) BeforeDelete(db *gorm.DB) (err error) {
	var pods []Pod
	db.Table("pods").Where("challenge_id = ?", c.ID).Find(&pods)
	for _, pod := range pods {
		db.Table("pods").Delete(&pod)
	}

	db.Table("flags").Where("challenge_id = ?", c.ID).Delete(&Flag{})
	db.Table("ports").Where("challenge_id = ?", c.ID).Delete(&Port{})
	db.Table("envs").Where("challenge_id = ?", c.ID).Delete(&Env{})
	db.Table("submissions").Where("challenge_id = ?", c.ID).Delete(&Submission{})
	db.Table("game_challenges").Where("challenge_id = ?", c.ID).Delete(&GameChallenge{})
	return nil
}
