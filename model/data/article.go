package data

import "time"

// Article 文章对象
type Article struct {
	// Id 自动生成的 uuid
	ArticleId string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	// Title 文章标题
	Title string `xorm:"'title' varchar(50) notnull" json:"title"`
	// Summary 文章概括
	Summary string `xorm:"text 'summary' notnull" json:"summary"`
	// Content 文章内容
	Content string `xorm:"text 'content' notnull" json:"content"`
	// AuthorId 作者的 id
	AuthorId string `xorm:"varchar(36) 'author_id' notnull" json:"author_id"`
	// ChallengeId 题目的 id
	ChallengeId string    `xorm:"varchar(36) 'challenge_id'" json:"challenge_id"`
	CreatedAt   time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt   time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
