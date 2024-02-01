package managers

import (
	"context"
	"errors"
	"fmt"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/go-connections/nat"
	"github.com/elabosak233/pgshub/containers/providers"
	"github.com/elabosak233/pgshub/utils/logger"
	"github.com/spf13/viper"
	"strconv"
	"sync"
	"time"
)

type DockerManager struct {
	InstanceId  int64
	RespId      string
	ImageName   string
	Port        int
	ExposedPort int
	FlagStr     string
	FlagEnv     string
	MemoryLimit int64   // MB
	CpuLimit    float64 // 核
	Duration    time.Duration
	CancelCtx   context.Context    // 存储可取消的上下文
	CancelFunc  context.CancelFunc // 存储取消函数
}

func NewDockerManagerImpl(imageName string, exposedPort int, flagStr string, flagEnv string, memoryLimit int64, cpuLimit float64, duration time.Duration) *DockerManager {
	return &DockerManager{
		ImageName:   imageName,
		ExposedPort: exposedPort,
		Duration:    duration,
		FlagStr:     flagStr,
		FlagEnv:     flagEnv,
		MemoryLimit: memoryLimit,
		CpuLimit:    cpuLimit,
	}
}

func (c *DockerManager) SetInstanceId(instanceId int64) {
	c.InstanceId = instanceId
}

func (c *DockerManager) Setup() (port int, err error) {
	port = providers.GetFreePort()
	if port == 0 {
		return 0, errors.New("未找到可用端口")
	}
	c.Port = port
	env := []string{fmt.Sprintf("%s=%s", c.FlagEnv, c.FlagStr)}
	containerConfig := &container.Config{
		Image: c.ImageName,
		Env:   env,
	}
	hostConfig := &container.HostConfig{
		PortBindings: nat.PortMap{
			nat.Port(strconv.Itoa(c.ExposedPort) + "/tcp"): []nat.PortBinding{
				{
					HostIP:   viper.GetString("container.docker.public_entry"),
					HostPort: strconv.Itoa(port),
				},
			},
		},
		Resources: container.Resources{
			Memory:   c.MemoryLimit * 1024 * 1024,
			NanoCPUs: int64(c.CpuLimit * 1e9),
		},
	}
	resp, err := providers.DockerClient.ContainerCreate(context.Background(), containerConfig, hostConfig, nil, nil, "")
	if err != nil {
		return 0, err
	}
	c.RespId = resp.ID
	err = providers.DockerClient.ContainerStart(context.Background(), c.RespId, types.ContainerStartOptions{})
	if err != nil {
		return 0, err
	}
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())
	return port, nil
}

func (c *DockerManager) GetContainerStatus() (status string, err error) {
	if providers.DockerClient == nil || c.RespId == "" {
		return "", errors.New("容器未创建或初始化失败")
	}
	resp, err := providers.DockerClient.ContainerInspect(context.Background(), c.RespId)
	if err != nil {
		return "removed", err
	}
	return resp.State.Status, err
}

func (c *DockerManager) RemoveAfterDuration(ctx context.Context) (success bool) {
	select {
	case <-time.After(c.Duration):
		_ = c.Remove()
		return true
	case <-ctx.Done(): // 当调用 cancelFunc 时，这里会接收到信号
		logger.Warn("容器移除被取消")
		return false
	}
}

func (c *DockerManager) Remove() (err error) {
	if providers.DockerClient == nil {
		return nil
	}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		errStop := providers.DockerClient.ContainerStop(context.Background(), c.RespId, container.StopOptions{})
		if errStop != nil {
		}
		// 等待容器停止
		_, errWait := providers.DockerClient.ContainerWait(context.Background(), c.RespId, container.WaitConditionNotRunning)
		if errWait != nil {
		}
		// 移除容器
		errRemove := providers.DockerClient.ContainerRemove(context.Background(), c.RespId, types.ContainerRemoveOptions{})
		if errRemove != nil {
		}
	}()
	wg.Wait()
	delete(providers.DockerPortsMap.M, c.Port)
	return err
}

func (c *DockerManager) Renew(duration time.Duration) (err error) {
	// 如果存在取消函数，则调用它来取消当前的移除操作
	if c.CancelFunc != nil {
		c.CancelFunc()
	}
	// 设置新的持续时间
	c.Duration = duration
	// 创建新的可取消上下文和取消函数
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration(c.CancelCtx)
	logger.Info("容器移除倒计时已重置")
	return nil
}
