package manager

import (
	"context"
	"errors"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/elabosak233/pgshub/internal/container/provider"
	"github.com/elabosak233/pgshub/internal/model"
	"go.uber.org/zap"
	"strconv"
	"time"
)

type DockerManager struct {
	InstanceID  uint
	RespID      string
	Image       string
	Inspect     types.ContainerJSON
	Port        int
	ExposedPort []*model.Port
	Env         []*model.Env
	MemoryLimit int64   // MB
	CPULimit    float64 // Core
	Duration    time.Duration
	CancelCtx   context.Context
	CancelFunc  context.CancelFunc
}

func NewDockerManager(imageName string, exposedPort []*model.Port, env []*model.Env, memoryLimit int64, cpuLimit float64, duration time.Duration) *DockerManager {
	return &DockerManager{
		Image:       imageName,
		ExposedPort: exposedPort,
		Duration:    duration,
		Env:         env,
		MemoryLimit: memoryLimit,
		CPULimit:    cpuLimit,
	}
}

func (c *DockerManager) SetInstanceID(instanceID uint) {
	c.InstanceID = instanceID
}

func (c *DockerManager) Setup() (assignedPorts nat.PortMap, err error) {
	var envs []string
	for _, env := range c.Env {
		envs = append(envs, fmt.Sprintf("%s=%s", env.Key, env.Value))
	}
	containerConfig := &container.Config{
		Image: c.Image,
		Env:   envs,
	}
	portBindings := make(nat.PortMap)
	for _, exposedPort := range c.ExposedPort {
		portStr := strconv.Itoa(exposedPort.Value) + "/tcp"
		portBindings[nat.Port(portStr)] = []nat.PortBinding{
			{
				HostIP:   "0.0.0.0",
				HostPort: "", // Let docker decide the port.
			},
		}
	}
	hostConfig := &container.HostConfig{
		PortBindings: portBindings,
		Resources: container.Resources{
			Memory:   c.MemoryLimit * 1024 * 1024,
			NanoCPUs: int64(c.CPULimit * 1e9),
		},
	}
	resp, err := provider.DockerCli().ContainerCreate(
		context.Background(),
		containerConfig,
		hostConfig,
		nil,
		nil,
		"",
	)
	if err != nil {
		panic(err)
	}
	c.RespID = resp.ID
	err = provider.DockerCli().ContainerStart(
		context.Background(),
		c.RespID,
		types.ContainerStartOptions{},
	)
	if err != nil {
		panic(err)
	}
	inspect, err := provider.DockerCli().ContainerInspect(
		context.Background(),
		c.RespID,
	)
	c.Inspect = inspect
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())

	assignedPorts = make(nat.PortMap)
	for port, bindings := range inspect.NetworkSettings.Ports {
		assignedPorts[port] = make([]nat.PortBinding, len(bindings))
		for i, binding := range bindings {
			assignedPorts[port][i] = nat.PortBinding{
				HostIP:   binding.HostIP,
				HostPort: binding.HostPort,
			}
		}
	}
	return assignedPorts, err
}

func (c *DockerManager) GetContainerStatus() (status string, err error) {
	if c.RespID == "" {
		return "", errors.New("容器未创建或初始化失败")
	}
	resp, err := provider.DockerCli().ContainerInspect(context.Background(), c.RespID)
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
		info, err := provider.DockerCli().ContainerInspect(context.Background(), c.RespID)
		if err != nil {
			return
		}

		if info.State.Running {
			_ = provider.DockerCli().ContainerStop(context.Background(), c.RespID, container.StopOptions{})              // Stop the container
			_, _ = provider.DockerCli().ContainerWait(context.Background(), c.RespID, container.WaitConditionNotRunning) // Wait for the container to stop
		}

		// Check if the container still exists before removing it
		_, err = provider.DockerCli().ContainerInspect(context.Background(), c.RespID)
		if err != nil && client.IsErrNotFound(err) {
			return // Instance not found, it has been removed
		}
		_ = provider.DockerCli().ContainerRemove(context.Background(), c.RespID, types.ContainerRemoveOptions{}) // Remove the container
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
			c.RespID,
		),
	)
}
