package manager

import (
	"context"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/container/provider"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/proxy"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"go.uber.org/zap"
	"strconv"
	"strings"
	"time"
)

type DockerManager struct {
	Images   []*model.Image
	Flag     model.Flag
	Duration time.Duration

	PodID      uint
	RespID     []string
	Proxy      *proxy.PodProxy
	Instances  []*model.Instance
	CancelCtx  context.Context
	CancelFunc context.CancelFunc
}

func NewDockerManager(images []*model.Image, flag model.Flag, duration time.Duration) ContainerManager {
	return &DockerManager{
		Images:   images,
		Duration: duration,
		Flag:     flag,
	}
}

func (c *DockerManager) SetPodID(podID uint) {
	c.PodID = podID
}

func (c *DockerManager) GetDuration() (duration time.Duration) {
	return c.Duration
}

func (c *DockerManager) Setup() (instances []*model.Instance, err error) {

	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())

	for _, image := range c.Images {
		envs := make([]string, 0)
		for _, env := range image.Envs {
			envs = append(envs, fmt.Sprintf("%s=%s", env.Key, env.Value))
		}
		envs = append(envs, fmt.Sprintf("%s=%s", c.Flag.Env, c.Flag.Value))

		containerConfig := &container.Config{
			Image: image.Name,
			Env:   envs,
		}

		portBindings := make(nat.PortMap)
		for _, port := range image.Ports {
			portStr := strconv.Itoa(port.Value) + "/tcp"
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
				Memory:   image.MemoryLimit * 1024 * 1024,
				NanoCPUs: int64(image.CPULimit * 1e9),
			},
		}

		resp, _ := provider.DockerCli().ContainerCreate(
			context.Background(),
			containerConfig,
			hostConfig,
			nil,
			nil,
			"",
		)

		c.RespID = append(c.RespID, resp.ID)

		// Start the container
		_ = provider.DockerCli().ContainerStart(
			context.Background(),
			c.RespID[len(c.RespID)-1],
			container.StartOptions{},
		)

		// Get the container's inspect information
		inspect, _ := provider.DockerCli().ContainerInspect(
			context.Background(),
			c.RespID[len(c.RespID)-1],
		)

		nats := make([]*model.Nat, 0)

		switch config.AppCfg().Container.Nat.Type {
		case "proxy":
			for port, bindings := range inspect.NetworkSettings.Ports {
				entries := make([]string, 0)
				for _, binding := range bindings {
					entries = append(entries, fmt.Sprintf(
						"%s:%d",
						config.AppCfg().Container.Docker.Entry,
						convertor.ToIntD(binding.HostPort, 0),
					))
				}
				c.Proxy = proxy.NewPodProxy(entries)
				c.Proxy.Start()
				for index, pp := range c.Proxy.Proxies {
					nats = append(nats, &model.Nat{
						SrcPort: port.Int(),
						DstPort: convertor.ToIntD(strings.Split(entries[index], ":")[1], 0),
						Proxy:   entries[index],
						Entry:   pp.Listen,
					})
				}
			}
		case "direct":
			for port, bindings := range inspect.NetworkSettings.Ports {
				for _, binding := range bindings {
					nats = append(nats, &model.Nat{
						SrcPort: port.Int(),
						DstPort: convertor.ToIntD(binding.HostPort, 0),
						Entry: fmt.Sprintf(
							"%s:%d",
							config.AppCfg().Container.Docker.Entry,
							convertor.ToIntD(binding.HostPort, 0),
						),
					})
				}
			}
		}

		instances = append(instances, &model.Instance{
			ImageID: image.ID,
			Nats:    nats,
		})
	}

	c.Instances = instances

	return instances, err
}

func (c *DockerManager) GetContainerStatus() (status string, err error) {
	status = "removed"
	for _, respID := range c.RespID {
		if resp, err := provider.DockerCli().ContainerInspect(context.Background(), respID); err == nil {
			status = resp.State.Status
		}
	}
	return status, err
}

func (c *DockerManager) RemoveAfterDuration() (success bool) {
	select {
	case <-time.After(c.Duration):
		c.Remove()
		return true
	case <-c.CancelCtx.Done():
		zap.L().Warn(fmt.Sprintf("[%s] Pod %d (RespID %s)'s removal plan has been cancelled.", color.InCyan("DOCKER"), c.PodID, c.RespID))
		return false
	}
}

func (c *DockerManager) Remove() {
	for _, respID := range c.RespID {
		go func(respID string) {
			// Check if the container is running before stopping it
			info, err := provider.DockerCli().ContainerInspect(context.Background(), respID)
			if err != nil {
				return
			}

			if info.State.Running {
				_ = provider.DockerCli().ContainerStop(context.Background(), respID, container.StopOptions{})              // Stop the container
				_, _ = provider.DockerCli().ContainerWait(context.Background(), respID, container.WaitConditionNotRunning) // Wait for the container to stop
			}

			// Check if the container still exists before removing it
			_, err = provider.DockerCli().ContainerInspect(context.Background(), respID)
			if err != nil && client.IsErrNotFound(err) {
				return // Instance not found, it has been removed
			}
			_ = provider.DockerCli().ContainerRemove(
				context.Background(),
				respID,
				container.RemoveOptions{},
			) // Remove the container
		}(respID)
	}
	if c.Proxy != nil {
		c.Proxy.Close()
	}
}

func (c *DockerManager) Renew(duration time.Duration) {
	if c.CancelFunc != nil {
		c.CancelFunc() // Calling the cancel function
	}
	c.Duration = duration
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration()
	zap.L().Warn(
		fmt.Sprintf("[%s] Pod %d (RespID %s) successfully renewed.",
			color.InCyan("DOCKER"),
			c.PodID,
			c.RespID,
		),
	)
}
