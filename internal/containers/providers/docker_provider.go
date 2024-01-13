package providers

import (
	"context"
	"fmt"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/internal"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/spf13/viper"
	"net"
	"strings"
	"time"
)

func NewDockerProvider() {
	dockerHost := viper.GetString("container.docker.host")
	dockerClient, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation(), client.WithHost(dockerHost))
	if err != nil {
		utils.Logger.Error("Docker 客户端初始化失败")
		panic(err)
	}
	utils.Logger.Info(fmt.Sprintf("Docker 客户端初始化成功，客户端版本 %s", dockerClient.ClientVersion()))
	internal.DockerClient = dockerClient // 注入全局变量
	version, err := dockerClient.ServerVersion(context.Background())
	if err != nil {
		utils.Logger.Error("Docker 服务端连接失败")
		panic(err)
	}
	utils.Logger.Info(fmt.Sprintf("Docker 远程服务端连接成功，服务端版本 %s", version.Version))
}

func GetFreePort() (port int) {
	if viper.GetString("container.docker.host") == ("unix:///var/run/docker.sock") || viper.GetString("container.docker.host") == "npipe:////./pipe/docker_engine" {
		for port := viper.GetInt("container.docker.ports.from"); port <= viper.GetInt("container.docker.ports.to"); port++ {
			addr := fmt.Sprintf("127.0.0.1:%d", port)
			if isPortAvailable(addr) {
				return port
			}
		}
	} else {
		for port := viper.GetInt("container.docker.ports.from"); port <= viper.GetInt("container.docker.ports.to"); port++ {
			addr := fmt.Sprintf("%s:%d", extractIP(viper.GetString("container.docker.host")), port)
			if isPortAvailable(addr) {
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
	conn, err := net.DialTimeout("tcp", addr, 1*time.Second)
	if err != nil {
		return true
	}
	defer func(conn net.Conn) {
		_ = conn.Close()
	}(conn)
	return false
}
