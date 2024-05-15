package proxy

import (
	"github.com/elabosak233/cloudsdale/internal/extension/config"
)

type IProxy interface {
	Setup()
	Close()
	Entry() (entry string)
}

func NewProxy(target string) IProxy {
	switch config.AppCfg().Container.Proxy.Type {
	case "tcp":
		return NewTCPProxy(target)
	case "ws":
		return NewWSProxy(target)
	}
	return nil
}
