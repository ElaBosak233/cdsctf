package model

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"gorm.io/gorm"
	"os"
	"path"
)

type Team struct {
	ID          uint    `json:"id"`                                                // The team's id. As primary key.
	Name        string  `gorm:"type:varchar(36);not null" json:"name"`             // The team's name.
	Description string  `gorm:"type:text" json:"description"`                      // The team's description.
	Email       string  `gorm:"type:varchar(64);" json:"email,omitempty"`          // The team's email.
	Avatar      *File   `gorm:"-" json:"avatar"`                                   // The team's avatar.
	CaptainID   uint    `gorm:"not null" json:"captain_id,omitempty"`              // The captain's id.
	Captain     *User   `json:"captain,omitempty"`                                 // The captain's user.
	IsLocked    *bool   `gorm:"not null;default:false" json:"is_locked,omitempty"` // Whether the team is locked. (true/false)
	InviteToken string  `gorm:"type:varchar(32);" json:"invite_token,omitempty"`   // The team's invite token.
	CreatedAt   int64   `gorm:"autoUpdateTime:milli" json:"created_at,omitempty"`  // The team's creation time.
	UpdatedAt   int64   `gorm:"autoUpdateTime:milli" json:"updated_at,omitempty"`  // The team's last update time.
	Users       []*User `gorm:"many2many:user_teams;" json:"users,omitempty"`      // The team's users.
}

func (t *Team) AfterFind(db *gorm.DB) (err error) {
	p := path.Join(utils.MediaPath, "teams", fmt.Sprintf("%d", t.ID))
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
	avatar := File{
		Name: name,
		Size: size,
	}
	t.Avatar = &avatar
	return nil
}

func (t *Team) BeforeDelete(db *gorm.DB) (err error) {
	db.Table("user_teams").Where("team_id = ?", t.ID).Delete(&UserTeam{})
	db.Table("game_teams").Where("team_id = ?", t.ID).Delete(&GameTeam{})
	return nil
}

func (t *Team) AfterCreate(db *gorm.DB) (err error) {
	db.Table("user_teams").Create(&UserTeam{
		TeamID: t.ID,
		UserID: t.CaptainID,
	})
	return nil
}

func (t *Team) AfterUpdate(db *gorm.DB) (err error) {
	var userTeams []UserTeam
	db.Table("user_teams").Where("team_id = ?", t.ID).Find(&userTeams)

	flag := true
	for _, userTeam := range userTeams {
		if userTeam.UserID == t.CaptainID {
			flag = false
		}
	}

	if flag {
		db.Table("user_teams").Create(&UserTeam{
			TeamID: t.ID,
			UserID: t.CaptainID,
		})
	}
	return nil
}
