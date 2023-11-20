package data

import "time"

// Challenge 题目对象
type Challenge struct {
	// Id 自动生成的 uuid
	Id string `xorm:"pk unique 'id' notnull" json:"id"`
	// Title 题目标题
	Title string `xorm:"varchar(50) 'title' notnull" json:"title"`
	// Description 题目描述
	Description string `xorm:"text 'description' notnull" json:"description"`
	// HasAttachment 是否有附件
	HasAttachment int `xorm:"int 'has_attachment'" json:"has_attachment"`
	// AttachmentId 附件 Id
	AttachmentId string `xorm:"json 'attachment_ids'" json:"attachment_id"`
	// IsDynamic 是否为动态题目
	IsDynamic int `xorm:"int 'is_dynamic'" json:"is_dynamic"`
	// InsidePort 题目内部端口号
	InsidePort int `xorm:"int 'inside_port'" json:"inside_port"`
	// ImageName 镜像名
	ImageName string `xorm:"text 'image_name'" json:"image_name"`
	// IsPractise 是否为训练题
	IsPractice int `xorm:"int 'is_practise'" json:"is_practice"`
	// Flag 期望的结果
	Flag string `xorm:"text 'flag'" json:"flag"`
	// UploaderId 上传者 Id
	UploaderId string `xorm:"text 'uploader_id' notnull" json:"uploader_id"`
	// Difficulty 难度
	Difficulty int       `xorm:"int 'difficulty'" json:"difficulty"`
	CreatedAt  time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt  time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
