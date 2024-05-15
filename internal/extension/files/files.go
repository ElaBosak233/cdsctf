// Package files
// Everything in this package is embedded.
// Everything in this package is read-only.
package files

import "embed"

var (
	//go:embed * statics/* templates/*
	FS embed.FS
)
