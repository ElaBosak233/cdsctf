package internal

import "github.com/docker/docker/client"

var InstanceMap = make(map[string]interface{})
var DockerClient *client.Client
