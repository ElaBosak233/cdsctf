package entity

type Article struct {
	ArticleID int64  `xorm:"'id' pk autoincr" json:"id"`               // The article's id. As primary key.
	Title     string `xorm:"'title' varchar(50) notnull" json:"title"` // The article's title.
	Summary   string `xorm:"'summary' text notnull" json:"summary"`    // The article's summary.
	Content   string `xorm:"'content' text notnull" json:"content"`    // The article's content.
	AuthorID  int64  `xorm:"'author_id' notnull" json:"author_id"`     // The article's author's id.
	CreatedAt int64  `xorm:"'created_at' created" json:"created_at"`   // The article's creation time.
	UpdatedAt int64  `xorm:"'updated_at' updated" json:"updated_at"`   // The article's last update time.
}
