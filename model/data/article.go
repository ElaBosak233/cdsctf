package data

import "time"

// Article 文章对象
type Article struct {
	// Id 自动生成的 uuid
	Id string `xorm:"pk unique 'id' notnull" json:"id"`
	// Title 文章标题
	Title string `xorm:"varchar(50) 'title' notnull" json:"title"`
	// Summary 文章概括
	Summary string `xorm:"text 'summary' not null" json:"summary"`
	// Content 文章内容
	Content string `xorm:"text 'content' not null" json:"content"`
	// AuthorId 作者的 id
	AuthorId string `xorm:"text 'author_id' not null" json:"author_id"`
	// ChallengeId 题目的 id
	ChallengeId string    `xorm:"text 'challenge_id'" json:"challenge_id"`
	CreatedAt   time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt   time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
