package global

import (
	"github.com/docker/docker/client"
	"sync"
)

// DockerClient 存储 Docker 客户端的地址
var DockerClient *client.Client

// InstanceMap 存储当前状态下所有的实例
var InstanceMap = make(map[any]interface{})

// DockerPortsMap 存储当前状态下端口占用状态
var DockerPortsMap = struct {
	sync.RWMutex
	M map[int]bool
}{M: make(map[int]bool)}
