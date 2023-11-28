package container

import (
	"context"
	"errors"
	"fmt"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/spf13/viper"
	"net"
	"strconv"
	"sync"
	"time"
)

type DockerContainer struct {
	Cli         *client.Client
	RespId      string
	ImageName   string
	ExposedPort int
	FlagStr     string
	FlagEnv     string
	MemoryLimit int64 // MB
	Duration    time.Duration
	Mu          sync.Mutex
	StopRenew   chan struct{} // 用于停止续期的信号通道
	Renewed     bool          // 标记是否已经进行过续期
}

func NewContainer(cli *client.Client, imageName string, exposedPort int, flagStr string, flagEnv string, memoryLimit int64, duration time.Duration) *DockerContainer {
	return &DockerContainer{
		Cli:         cli,
		ImageName:   imageName,
		ExposedPort: exposedPort,
		Duration:    duration,
		FlagStr:     flagStr,
		FlagEnv:     flagEnv,
		MemoryLimit: memoryLimit,
	}
}

func getAvailablePort() int {
	for port := viper.GetInt("Container.Ports.From"); port <= viper.GetInt("Container.Ports.To"); port++ {
		addr := fmt.Sprintf(":%d", port)
		l, err := net.Listen("tcp", addr)
		if err == nil {
			_ = l.Close()
			return port
		}
	}
	return 0
}

func (c *DockerContainer) Setup() error {
	port := getAvailablePort()
	if port == 0 {
		return errors.New("未找到可用端口")
	}
	cli, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
	if err != nil {
		return errors.New("客户端创建失败")
	}
	env := []string{fmt.Sprintf("%s=%s", c.FlagEnv, c.FlagStr)}
	containerConfig := &container.Config{
		Image: c.ImageName,
		Env:   env,
	}
	hostConfig := &container.HostConfig{
		PortBindings: nat.PortMap{
			nat.Port(strconv.Itoa(c.ExposedPort) + "/tcp"): []nat.PortBinding{
				{
					HostIP:   viper.GetString("Container.Host"),
					HostPort: strconv.Itoa(port),
				},
			},
		},
		Resources: container.Resources{
			Memory: c.MemoryLimit * 1024 * 1024,
		},
	}
	resp, err := cli.ContainerCreate(context.Background(), containerConfig, hostConfig, nil, nil, "")
	if err != nil {
		return err
	}
	c.RespId = resp.ID
	err = cli.ContainerStart(context.Background(), c.RespId, types.ContainerStartOptions{})
	if err != nil {
		return err
	}
	c.ShutDownAfterDuration()
	return nil
}

func (c *DockerContainer) GetContainerStatus() (string, error) {
	if c.Cli == nil || c.RespId == "" {
		return "", errors.New("容器未创建或初始化失败")
	}
	resp, err := c.Cli.ContainerInspect(context.Background(), c.RespId)
	if err != nil {
		return "removed", err
	}
	return resp.State.Status, nil
}

func (c *DockerContainer) ShutDownAfterDuration() {
	time.AfterFunc(c.Duration, func() {
		_ = c.ShutDown()
	})
}

func (c *DockerContainer) ShutDown() error {
	if c.Cli == nil {
		return nil
	}
	err := c.Cli.ContainerStop(context.Background(), c.RespId, container.StopOptions{})
	if err != nil {
		return err
	}
	statusCh, errCh := c.Cli.ContainerWait(context.Background(), c.RespId, container.WaitConditionNotRunning)
	select {
	case err := <-errCh:
		if err != nil {
			return err
		}
	case <-statusCh:
	}
	err = c.Cli.ContainerRemove(context.Background(), c.RespId, types.ContainerRemoveOptions{})
	if err != nil {
		return err
	}
	return nil
}

func (c *DockerContainer) Renew(duration time.Duration) error {
	c.Mu.Lock()
	defer c.Mu.Unlock()
	if c.Renewed {
		return errors.New("靶机已续期")
	}
	c.Duration += duration
	time.AfterFunc(c.Duration, func() {
		c.Mu.Lock()
		defer c.Mu.Unlock()
		if !c.Renewed {
			_ = c.ShutDown()
		}
	})
	c.Renewed = true
	return nil
}
