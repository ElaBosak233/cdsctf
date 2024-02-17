package proxy

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/phayes/freeport"
	"go.uber.org/zap"
	"net"
)

type PodProxy struct {
	Proxies []*SingleProxy
}

type SingleProxy struct {
	Target   string
	Listen   string
	listener net.Listener
}

func NewPodProxy(targets []string) *PodProxy {
	instanceProxies := make([]*SingleProxy, 0)
	for _, target := range targets {
		instanceProxies = append(instanceProxies, &SingleProxy{
			Target: target,
		})
	}
	return &PodProxy{
		Proxies: instanceProxies,
	}
}

func (p *PodProxy) Start() {
	for _, instanceProxy := range p.Proxies {
		port, err := freeport.GetFreePort()
		if err != nil {
			zap.L().Error("Failed to get free port for proxy.", zap.Error(err))
		}
		instanceProxy.Listen = fmt.Sprintf("%s:%d", config.AppCfg().Container.Nat.Entry, port)
		instanceProxy.listener, err = net.Listen("tcp", instanceProxy.Listen)
		if err != nil {
			zap.L().Error(fmt.Sprintf("Failed to listen on %s: %v", instanceProxy.Listen, err))
		}

		zap.L().Info(fmt.Sprintf("Proxy listening on %s, forwarding to %s", instanceProxy.Listen, instanceProxy.Target))
		go func(i *SingleProxy) {
			for {
				conn, err := i.listener.Accept()
				if err != nil {
					if errors.Is(err, net.ErrClosed) {
						return // listener 已经关闭
					}
					zap.L().Error(fmt.Sprintf("Failed to accept connection: %v", err))
					continue
				}
				switch config.AppCfg().Container.TrafficCapture.Enabled {
				case true:
					go HandleInTrafficCapture(conn, i.Target)
				case false:
					go Handle(conn, i.Target)
				}
			}
		}(instanceProxy)
	}
}

func (p *PodProxy) Close() {
	for _, instanceProxy := range p.Proxies {
		_ = instanceProxy.listener.Close()
	}
}
