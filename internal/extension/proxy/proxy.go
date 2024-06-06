package proxy

type IProxy interface {
	Setup()
	Close()
	Entry() (entry string)
}

func NewProxy(target string) IProxy {
	return NewWSProxy(target)
}
