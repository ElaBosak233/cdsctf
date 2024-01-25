package managers

import (
	"context"
	"errors"
	"fmt"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/go-connections/nat"
	"github.com/elabosak233/pgshub"
	"github.com/elabosak233/pgshub/containers/providers"
	"github.com/elabosak233/pgshub/utils"
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
	cancelCtx   context.Context    // 存储可取消的上下文
	cancelFunc  context.CancelFunc // 存储取消函数
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
	resp, err := global.DockerClient.ContainerCreate(context.Background(), containerConfig, hostConfig, nil, nil, "")
	if err != nil {
		return 0, err
	}
	c.RespId = resp.ID
	err = global.DockerClient.ContainerStart(context.Background(), c.RespId, types.ContainerStartOptions{})
	if err != nil {
		return 0, err
	}
	c.cancelCtx, c.cancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration(c.cancelCtx)
	return port, nil
}

func (c *DockerManager) GetContainerStatus() (status string, err error) {
	if global.DockerClient == nil || c.RespId == "" {
		return "", errors.New("容器未创建或初始化失败")
	}
	resp, err := global.DockerClient.ContainerInspect(context.Background(), c.RespId)
	if err != nil {
		return "removed", err
	}
	return resp.State.Status, err
}

func (c *DockerManager) RemoveAfterDuration(ctx context.Context) {
	select {
	case <-time.After(c.Duration):
		_ = c.Remove()
	case <-ctx.Done(): // 当调用 cancelFunc 时，这里会接收到信号
		utils.Logger.Warn("容器移除被取消")
		return
	}
}

func (c *DockerManager) Remove() (err error) {
	if global.DockerClient == nil {
		return nil
	}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		errStop := global.DockerClient.ContainerStop(context.Background(), c.RespId, container.StopOptions{})
		if errStop != nil {
		}
		// 等待容器停止
		_, errWait := global.DockerClient.ContainerWait(context.Background(), c.RespId, container.WaitConditionNotRunning)
		if errWait != nil {
		}
		// 移除容器
		errRemove := global.DockerClient.ContainerRemove(context.Background(), c.RespId, types.ContainerRemoveOptions{})
		if errRemove != nil {
		}
	}()
	wg.Wait()
	delete(global.InstanceMap, c.InstanceId)
	delete(global.DockerPortsMap.M, c.Port)
	return err
}

func (c *DockerManager) Renew(duration time.Duration) (err error) {
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
