package entity

// Article 文章对象
type Article struct {
	ArticleId   int64  `xorm:"'id' pk autoincr" json:"id"`
	Title       string `xorm:"'title' varchar(50) notnull" json:"title"`
	Summary     string `xorm:"text 'summary' notnull" json:"summary"`
	Content     string `xorm:"text 'content' notnull" json:"content"`
	AuthorId    string `xorm:"varchar(36) 'author_id' notnull" json:"author_id"`
	ChallengeId string `xorm:"varchar(36) 'challenge_id'" json:"challenge_id"`
	CreatedAt   int64  `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt   int64  `xorm:"updated 'updated_at'" json:"updated_at"`
}
