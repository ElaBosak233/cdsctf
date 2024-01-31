package providers

import (
	"context"
	"fmt"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/globals"
	"github.com/elabosak233/pgshub/utils/logger"
	"github.com/spf13/viper"
	"net"
	"strconv"
	"strings"
	"time"
)

func NewDockerProvider() {
	dockerUri := viper.GetString("container.docker.uri")
	dockerClient, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation(), client.WithHost(dockerUri))
	if err != nil {
		logger.Error("Docker 客户端初始化失败")
		panic(err)
	}
	logger.Info(fmt.Sprintf("Docker 客户端初始化成功，客户端版本 %s", dockerClient.ClientVersion()))
	globals.DockerClient = dockerClient // 注入全局变量
	version, err := dockerClient.ServerVersion(context.Background())
	if err != nil {
		logger.Error("Docker 服务端连接失败")
		panic(err)
	}
	logger.Info(fmt.Sprintf("Docker 远程服务端连接成功，服务端版本 %s", version.Version))
}

func GetFreePort() (port int) {
	globals.DockerPortsMap.Lock()
	defer globals.DockerPortsMap.Unlock()
	portFrom := viper.GetInt("container.docker.ports.from")
	portTo := viper.GetInt("container.docker.ports.to")
	if viper.GetString("container.docker.uri") == ("unix:///var/run/docker.sock") || viper.GetString("container.docker.uri") == "npipe:////./pipe/docker_engine" {
		for port := portFrom; port <= portTo; port++ {
			addr := fmt.Sprintf("127.0.0.1:%d", port)
			if isPortAvailable(addr) && !globals.DockerPortsMap.M[port] {
				globals.DockerPortsMap.M[port] = true
				return port
			}
		}
	} else {
		for port := portFrom; port <= portTo; port++ {
			addr := fmt.Sprintf("%s:%d", extractIP(viper.GetString("container.docker.uri")), port)
			if isPortAvailable(addr) && !globals.DockerPortsMap.M[port] {
				globals.DockerPortsMap.M[port] = true
				return port
			}
		}
	}
	return 0
}

func extractIP(input string) (host string) {
	parts := strings.Split(input, "://")
	if len(parts) != 2 {
		return ""
	}
	ipAndPort := parts[1]
	ipParts := strings.Split(ipAndPort, ":")
	if len(ipParts) != 2 {
		return ""
	}
	return ipParts[0]
}

func isPortAvailable(addr string) bool {
	host, port, err := net.SplitHostPort(addr)
	if err != nil {
		return false
	}
	if _, err := strconv.Atoi(port); err != nil {
		return false
	}
	conn, err := net.DialTimeout("tcp", net.JoinHostPort(host, port), 1*time.Second)
	if err != nil {
		return true
	}
	defer func(conn net.Conn) {
		_ = conn.Close()
	}(conn)
	return false
}
