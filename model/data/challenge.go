/*
题目
*/

package data

import "time"

type Challenge struct {
	Id           string    `xorm:"pk unique 'id' notnull"`
	Title        string    `xorm:"varchar(50) 'title' notnull"`
	Description  string    `xorm:"text 'description'"`
	AttachmentId string    `xorm:"json 'attachment_ids'"`
	UploaderId   string    `xorm:"text 'uploader_id' notnull"`
	Difficulty   string    `xorm:"text 'difficulty' notnull"`
	ArticleIds   []string  `xorm:"json 'article_ids'"`
	CreatedAt    time.Time `xorm:"created 'created_at'"`
	UpdatedAt    time.Time `xorm:"updated 'updated_at'"`
}
