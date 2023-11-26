package data

import "time"

// Challenge 题目
type Challenge struct {
	ChallengeId string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	// Title 标题
	Title string `xorm:"varchar(50) 'title' notnull" json:"title"`
	// Description 描述
	Description string `xorm:"text 'description' notnull" json:"description"`
	// AttachmentId 附件 Id
	AttachmentId string `xorm:"json 'attachment_ids'" json:"attachment_id"`
	// IsDynamic 是否为动态题目
	IsDynamic int `xorm:"int 'is_dynamic'" json:"is_dynamic" binding:"oneof=0 1"`
	// InsidePort 内部端口号
	InsidePort int `xorm:"int 'inside_port'" json:"inside_port"`
	// ImageName 镜像名
	ImageName string `xorm:"text 'image_name'" json:"image_name"`
	// Flag 期望的结果
	Flag string `xorm:"text 'flag'" json:"flag"`
	// UploaderId 上传者 Id
	UploaderId string `xorm:"varchar(36) 'uploader_id' notnull" json:"uploader_id"`
	// Difficulty 难度
	Difficulty int       `xorm:"int 'difficulty'" json:"difficulty"`
	CreatedAt  time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt  time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
