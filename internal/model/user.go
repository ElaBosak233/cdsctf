package model

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"gorm.io/gorm"
	"os"
	"path"
)

type User struct {
	ID          uint    `json:"id"`                                                                      // The user's id. As primary key.
	Username    string  `gorm:"column:username;type:varchar(16);unique;not null;index;" json:"username"` // The user's username. As a unique identifier.
	Nickname    string  `gorm:"column:nickname;type:varchar(36);not null" json:"nickname"`               // The user's nickname. Not unique.
	Description string  `gorm:"column:description;type:text" json:"description"`                         // The user's description.
	Email       string  `gorm:"column:email;varchar(64);unique;not null" json:"email,omitempty"`         // The user's email.
	Avatar      *File   `gorm:"-" json:"avatar"`                                                         // The user's avatar.
	Group       string  `gorm:"column:group;varchar(16);not null;" json:"group,omitempty"`               // The user's group.
	Password    string  `gorm:"column:password;type:varchar(255);not null" json:"password,omitempty"`    // The user's password. Crypt.
	CreatedAt   int64   `gorm:"autoUpdateTime:milli" json:"created_at,omitempty"`                        // The user's creation time.
	UpdatedAt   int64   `gorm:"autoUpdateTime:milli" json:"updated_at,omitempty"`                        // The user's last update time.
	Teams       []*Team `gorm:"many2many:user_teams;" json:"teams,omitempty"`                            // The user's teams.
}

func (u *User) Simplify() {
	u.Password = ""
	u.Description = ""
}

func (u *User) AfterFind(db *gorm.DB) (err error) {
	p := path.Join(utils.MediaPath, "users", fmt.Sprintf("%d", u.ID))
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
	u.Avatar = &avatar
	return nil
}

func (u *User) BeforeDelete(db *gorm.DB) (err error) {
	db.Table("user_teams").Where("user_id = ?", u.ID).Delete(&UserTeam{})
	return nil
}
