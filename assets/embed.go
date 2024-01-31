package embed

import "embed"

var (
	//go:embed statics/* templates/*
	FS embed.FS
)
