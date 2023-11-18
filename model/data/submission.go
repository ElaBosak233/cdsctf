package data

type Submission struct {
	Answer string `xorm:"varchar(128) 'answer' notnull" json:"answer"`
}
