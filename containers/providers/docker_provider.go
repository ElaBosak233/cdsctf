package providers

import (
	"context"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/docker/docker/client"
	"github.com/spf13/viper"
	"go.uber.org/zap"
)

var (
	// DockerCli Store Docker client pointers
	DockerCli *client.Client
)

func NewDockerProvider() {
	dockerUri := viper.GetString("container.docker.uri")
	dockerClient, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation(), client.WithHost(dockerUri))
	if err != nil {
		zap.L().Error("Docker client initialization failed.")
		panic(err)
	}
	zap.L().Info(fmt.Sprintf("Docker client initialization successful, client version %s", color.InCyan(dockerClient.ClientVersion())))
	DockerCli = dockerClient // Inject into global variable DockerCli
	version, err := dockerClient.ServerVersion(context.Background())
	if err != nil {
		zap.L().Error("Docker server connection failure.")
		panic(err)
	}
	zap.L().Info(fmt.Sprintf("Docker remote server connection successful, server version %s", color.InCyan(version.Version)))
}
