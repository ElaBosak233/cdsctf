package proxy

type IPodProxy interface {
	Setup()
	Close()
	Proxies() []IProxy
}

type PodProxy struct {
	proxies []IProxy
}

func NewPodProxy(targets []string) IPodProxy {
	instanceProxies := make([]IProxy, 0)
	for _, target := range targets {
		instanceProxies = append(instanceProxies, NewProxy(target))
	}
	return &PodProxy{
		proxies: instanceProxies,
	}
}

func (p *PodProxy) Setup() {
	for _, instanceProxy := range p.proxies {
		instanceProxy.Setup()
	}
}

func (p *PodProxy) Proxies() []IProxy {
	return p.proxies
}

func (p *PodProxy) Close() {
	for _, instanceProxy := range p.proxies {
		instanceProxy.Close()
	}
}
