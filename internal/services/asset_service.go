package services

type AssetService interface {
	GetUserAvatarList() (res []string, err error)
	GetTeamAvatarList() (res []string, err error)
}
