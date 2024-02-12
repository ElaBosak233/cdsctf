package providers

import (
	"context"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/utils/config"
	"go.uber.org/zap"
)

var (
	// dockerCli Store Docker client pointers
	dockerCli *client.Client
)

func DockerCli() *client.Client {
	return dockerCli
}

func NewDockerProvider() {
	dockerUri := config.Cfg().Container.Docker.URI
	dockerClient, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation(), client.WithHost(dockerUri))
	if err != nil {
		zap.L().Fatal("Docker client initialization failed.")
		panic(err)
	}
	zap.L().Info(fmt.Sprintf("Docker client initialization successful, client version %s", color.InCyan(dockerClient.ClientVersion())))
	dockerCli = dockerClient
	version, err := dockerClient.ServerVersion(context.Background())
	if err != nil {
		zap.L().Fatal("Docker server connection failure.")
		panic(err)
	}
	zap.L().Info(fmt.Sprintf("Docker remote server connection successful, server version %s", color.InCyan(version.Version)))
}
