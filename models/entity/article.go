package entity

type Article struct {
	ArticleId int64  `xorm:"'id' pk autoincr" json:"id"`                       // The article's id. As primary key.
	Title     string `xorm:"'title' varchar(50) notnull" json:"title"`         // The article's title.
	Summary   string `xorm:"text 'summary' notnull" json:"summary"`            // The article's summary.
	Content   string `xorm:"text 'content' notnull" json:"content"`            // The article's content.
	AuthorId  string `xorm:"varchar(36) 'author_id' notnull" json:"author_id"` // The article's author's id.
	CreatedAt int64  `xorm:"created 'created_at'" json:"created_at"`           // The article's creation time.
	UpdatedAt int64  `xorm:"updated 'updated_at'" json:"updated_at"`           // The article's last update time.
}
