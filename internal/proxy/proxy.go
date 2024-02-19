package proxy

import (
	"github.com/elabosak233/cloudsdale/internal/config"
)

type IProxy interface {
	Setup()
	Close()
	GetEntry() (entry string)
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

type PodProxy struct {
	Proxies []IProxy
}

func NewPodProxy(targets []string) *PodProxy {
	instanceProxies := make([]IProxy, 0)
	for _, target := range targets {
		instanceProxies = append(instanceProxies, NewProxy(target))
	}
	return &PodProxy{
		Proxies: instanceProxies,
	}
}

func (p *PodProxy) Start() {
	for _, instanceProxy := range p.Proxies {
		instanceProxy.Setup()
	}
}

func (p *PodProxy) Close() {
	for _, instanceProxy := range p.Proxies {
		instanceProxy.Close()
	}
}
