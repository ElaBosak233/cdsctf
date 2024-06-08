package utils

// Need to be injected by -ldflags
var (
	GitCommitID = "N/A"
	GitBranch   = "N/A"
	GitTag      = "N/A"
)

const (
	ConfigsPath  = "./configs"
	MediaPath    = "./media"
	FilesPath    = "./files"
	CapturesPath = "./captures"
	FrontendPath = "./dist"
)

var (
	True  = true
	False = false
)
