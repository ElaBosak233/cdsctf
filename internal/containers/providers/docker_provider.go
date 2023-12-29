package providers

import (
	"context"
	"fmt"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/internal"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/spf13/viper"
	"net"
	"os"
	"strings"
)

func NewDockerProvider() {
	dockerHost := viper.GetString("container.docker.host")
	dockerClient, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation(), client.WithHost(dockerHost))
	if err != nil {
		utils.Logger.Error("Docker 客户端初始化失败")
		os.Exit(1)
	}
	utils.Logger.Info(fmt.Sprintf("Docker 客户端初始化成功，客户端版本 %s", dockerClient.ClientVersion()))
	internal.DockerClient = dockerClient // 注入全局变量
	version, err := dockerClient.ServerVersion(context.Background())
	if err != nil {
		utils.Logger.Error("Docker 服务端连接失败")
		os.Exit(1)
	}
	utils.Logger.Info(fmt.Sprintf("Docker 远程服务端连接成功，服务端版本 %s", version.Version))
}

func GetFreePort() (port int) {
	if viper.GetString("container.docker.host") == ("unix:///var/run/docker.sock") || viper.GetString("container.docker.host") == "npipe:////./pipe/docker_engine" {
		for port := viper.GetInt("container.docker.ports.from"); port <= viper.GetInt("container.docker.ports.to"); port++ {
			addr := fmt.Sprintf("127.0.0.1:%d", port)
			l, err := net.Listen("tcp", addr)
			if err == nil {
				_ = l.Close()
				return port
			}
		}
	} else {
		for port := viper.GetInt("container.docker.ports.from"); port <= viper.GetInt("container.docker.ports.to"); port++ {
			addr := fmt.Sprintf("%s:%d", strings.Split(viper.GetString("container.docker.host"), ":")[0], port)
			l, err := net.Listen("tcp", addr)
			if err == nil {
				_ = l.Close()
				return port
			}
		}
	}
	return 0
}
