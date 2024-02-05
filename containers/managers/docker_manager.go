package managers

import (
	"context"
	"errors"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/elabosak233/pgshub/containers/providers"
	"github.com/elabosak233/pgshub/utils/convertor"
	"go.uber.org/zap"
	"strconv"
	"time"
)

type DockerManager struct {
	InstanceID  int64
	RespID      string
	Image       string
	Inspect     types.ContainerJSON
	Port        int
	ExposedPort int
	FlagStr     string
	FlagEnv     string
	MemoryLimit int64   // MB
	CpuLimit    float64 // Core
	Duration    time.Duration
	CancelCtx   context.Context
	CancelFunc  context.CancelFunc
}

func NewDockerManagerImpl(imageName string, exposedPort int, flagStr string, flagEnv string, memoryLimit int64, cpuLimit float64, duration time.Duration) *DockerManager {
	return &DockerManager{
		Image:       imageName,
		ExposedPort: exposedPort,
		Duration:    duration,
		FlagStr:     flagStr,
		FlagEnv:     flagEnv,
		MemoryLimit: memoryLimit,
		CpuLimit:    cpuLimit,
	}
}

func (c *DockerManager) SetInstanceId(instanceId int64) {
	c.InstanceID = instanceId
}

func (c *DockerManager) Setup() (port int, err error) {
	env := []string{fmt.Sprintf("%s=%s", c.FlagEnv, c.FlagStr)}
	containerConfig := &container.Config{
		Image: c.Image,
		Env:   env,
	}
	hostConfig := &container.HostConfig{
		PortBindings: nat.PortMap{
			nat.Port(strconv.Itoa(c.ExposedPort) + "/tcp"): []nat.PortBinding{
				{
					HostIP:   "0.0.0.0",
					HostPort: "", // Let docker decide the port.
				},
			},
		},
		Resources: container.Resources{
			Memory:   c.MemoryLimit * 1024 * 1024,
			NanoCPUs: int64(c.CpuLimit * 1e9),
		},
	}
	resp, err := providers.DockerCli.ContainerCreate(context.Background(), containerConfig, hostConfig, nil, nil, "")
	c.RespID = resp.ID
	err = providers.DockerCli.ContainerStart(context.Background(), c.RespID, types.ContainerStartOptions{})
	inspect, err := providers.DockerCli.ContainerInspect(context.Background(), c.RespID)
	c.Inspect = inspect
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())
	return convertor.ToIntD(inspect.NetworkSettings.Ports[nat.Port(strconv.Itoa(c.ExposedPort)+"/tcp")][0].HostPort, 0), err
}

func (c *DockerManager) GetContainerStatus() (status string, err error) {
	if c.RespID == "" {
		return "", errors.New("容器未创建或初始化失败")
	}
	resp, err := providers.DockerCli.ContainerInspect(context.Background(), c.RespID)
	if err != nil {
		return "removed", err
	}
	return resp.State.Status, err
}

func (c *DockerManager) RemoveAfterDuration(ctx context.Context) (success bool) {
	select {
	case <-time.After(c.Duration):
		c.Remove()
		return true
	case <-ctx.Done():
		zap.L().Warn(fmt.Sprintf("[%s] Instance %d (RespID %s)'s removal plan has been cancelled.", color.InCyan("DOCKER"), c.InstanceID, c.RespID))
		return false
	}
}

func (c *DockerManager) Remove() {
	go func() {
		// Check if the container is running before stopping it
		info, err := providers.DockerCli.ContainerInspect(context.Background(), c.RespID)
		if err != nil {
			return
		}

		if info.State.Running {
			_ = providers.DockerCli.ContainerStop(context.Background(), c.RespID, container.StopOptions{})              // Stop the container
			_, _ = providers.DockerCli.ContainerWait(context.Background(), c.RespID, container.WaitConditionNotRunning) // Wait for the container to stop
		}

		// Check if the container still exists before removing it
		_, err = providers.DockerCli.ContainerInspect(context.Background(), c.RespID)
		if err != nil && client.IsErrNotFound(err) {
			return // Container not found, it has been removed
		}
		_ = providers.DockerCli.ContainerRemove(context.Background(), c.RespID, types.ContainerRemoveOptions{}) // Remove the container
	}()
}

func (c *DockerManager) Renew(duration time.Duration) {
	if c.CancelFunc != nil {
		c.CancelFunc() // Calling the cancel function
	}
	c.Duration = duration
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration(c.CancelCtx)
	zap.L().Warn(
		fmt.Sprintf("[%s] Instance %d (RespID %s) successfully renewed.",
			color.InCyan("DOCKER"),
			c.InstanceID,
			c.RespID))
}
