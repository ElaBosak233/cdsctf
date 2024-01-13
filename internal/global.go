package internal

import "github.com/docker/docker/client"

var InstanceMap = make(map[any]interface{})
var DockerClient *client.Client
