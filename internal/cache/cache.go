package cache

type ICache interface {
	Get(key string) (value string, err error)
	Set(key string, value string, expire int) (err error)
}

func NewCache() ICache {
	return nil
}
