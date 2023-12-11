package container

import (
	"context"
	"errors"
	"fmt"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/elabosak233/pgshub/utils"
	"github.com/spf13/viper"
	"net"
	"strconv"
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
	cancelCtx   context.Context    // 存储可取消的上下文
	cancelFunc  context.CancelFunc // 存储取消函数
}

func NewDockerContainer(cli *client.Client, imageName string, exposedPort int, flagStr string, flagEnv string, memoryLimit int64, duration time.Duration) *DockerContainer {
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
	for port := viper.GetInt("Container.Docker.Ports.From"); port <= viper.GetInt("Container.Docker.Ports.To"); port++ {
		addr := fmt.Sprintf(":%d", port)
		l, err := net.Listen("tcp", addr)
		if err == nil {
			_ = l.Close()
			return port
		}
	}
	return 0
}

func (c *DockerContainer) Setup() (port int, error error) {
	port = getAvailablePort()
	if port == 0 {
		return 0, errors.New("未找到可用端口")
	}
	cli, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
	if err != nil {
		return 0, errors.New("客户端创建失败")
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
					HostIP:   viper.GetString("Container.Docker.Entry"),
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
		return 0, err
	}
	c.RespId = resp.ID
	err = cli.ContainerStart(context.Background(), c.RespId, types.ContainerStartOptions{})
	if err != nil {
		return 0, err
	}
	c.cancelCtx, c.cancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration(c.cancelCtx)
	return port, nil
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

func (c *DockerContainer) RemoveAfterDuration(ctx context.Context) {
	select {
	case <-time.After(c.Duration):
		_ = c.Remove()
	case <-ctx.Done(): // 当调用 cancelFunc 时，这里会接收到信号
		utils.Logger.Warn("容器移除被取消")
		return
	}
}

func (c *DockerContainer) Remove() error {
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
	// 如果存在取消函数，则调用它来取消当前的移除操作
	if c.cancelFunc != nil {
		c.cancelFunc()
	}
	// 设置新的持续时间
	c.Duration = duration
	// 创建新的可取消上下文和取消函数
	c.cancelCtx, c.cancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration(c.cancelCtx)
	utils.Logger.Info("容器移除倒计时已重置")
	return nil
}
