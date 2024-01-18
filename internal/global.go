package internal

import (
	"github.com/docker/docker/client"
	"sync"
)

// DockerClient 存储 Docker 客户端的地址
var DockerClient *client.Client

// InstanceMap 存储当前状态下所有的实例
var InstanceMap = make(map[any]interface{})

// UserInstanceRequestMap 用于存储用户上次请求的时间
var UserInstanceRequestMap = struct {
	sync.RWMutex
	m map[int64]int64
}{m: make(map[int64]int64)}

// GetUserInstanceRequestMap 返回用户上次请求的时间
func GetUserInstanceRequestMap(userId int64) int64 {
	UserInstanceRequestMap.RLock()
	defer UserInstanceRequestMap.RUnlock()
	return UserInstanceRequestMap.m[userId]
}

// SetUserInstanceRequestMap 设置用户上次请求的时间
func SetUserInstanceRequestMap(userId int64, t int64) {
	UserInstanceRequestMap.Lock()
	defer UserInstanceRequestMap.Unlock()
	UserInstanceRequestMap.m[userId] = t
}
