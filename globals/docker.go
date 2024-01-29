package globals

import (
	"github.com/docker/docker/client"
	"sync"
)

// DockerClient 存储 Docker 客户端指针
var DockerClient *client.Client

// DockerPortsMap 存储当前状态下端口占用状态
var DockerPortsMap = struct {
	sync.RWMutex
	M map[int]bool
}{M: make(map[int]bool)}
