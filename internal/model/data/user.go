/*
用户
*/

package data

import "time"

type User struct {
	// Id 自动生成的 uuid
	Id string `xorm:"pk unique 'id' notnull"`
	// Username 用户名
	Username string `xorm:"varchar(50) unique 'username' notnull"`
	// Password 密码哈希
	Password string `xorm:"varchar(255) 'password' notnull"`
	// GroupIds 所属用户组
	GroupIds []string `xorm:"json 'group_ids'"`
	// TeamIds 所属团队
	TeamIds []string `xorm:"json 'team_ids'"`
	// ArticleIds 撰写的文章
	ArticleIds []string  `xorm:"json 'article_ids'"`
	CreatedAt  time.Time `xorm:"created 'created_at'"`
	UpdatedAt  time.Time `xorm:"updated 'updated_at'"`
}
