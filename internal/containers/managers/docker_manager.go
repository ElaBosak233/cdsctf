package managers

import (
	"context"
	"errors"
	"fmt"
	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/elabosak233/pgshub/internal"
	"github.com/elabosak233/pgshub/internal/containers/providers"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/spf13/viper"
	"strconv"
	"sync"
	"time"
)

var portMutex sync.Mutex // 互斥锁

type DockerManager struct {
	InstanceId  string
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

func NewDockerManagerImpl(instanceId string, imageName string, exposedPort int, flagStr string, flagEnv string, memoryLimit int64, duration time.Duration) *DockerManager {
	return &DockerManager{
		InstanceId:  instanceId,
		ImageName:   imageName,
		ExposedPort: exposedPort,
		Duration:    duration,
		FlagStr:     flagStr,
		FlagEnv:     flagEnv,
		MemoryLimit: memoryLimit,
	}
}

func (c *DockerManager) Setup() (port int, err error) {
	portMutex.Lock()         // 获取锁
	defer portMutex.Unlock() // 确保在函数退出时释放锁
	port = providers.GetFreePort()
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
					HostIP:   viper.GetString("container.docker.public_entry"),
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

func (c *DockerManager) GetContainerStatus() (status string, err error) {
	if internal.DockerClient == nil || c.RespId == "" {
		return "", errors.New("容器未创建或初始化失败")
	}
	resp, err := internal.DockerClient.ContainerInspect(context.Background(), c.RespId)
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
	if internal.DockerClient == nil {
		return nil
	}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		// Stop the container
		errStop := internal.DockerClient.ContainerStop(context.Background(), c.RespId, container.StopOptions{})
		if errStop != nil {
			// Handle error if needed
		}
		// Wait for the container to stop
		_, errWait := internal.DockerClient.ContainerWait(context.Background(), c.RespId, container.WaitConditionNotRunning)
		if errWait != nil {
			// Handle error if needed
		}
		// Remove the container
		errRemove := internal.DockerClient.ContainerRemove(context.Background(), c.RespId, types.ContainerRemoveOptions{})
		if errRemove != nil {
			// Handle error if needed
		}
	}()
	wg.Wait()
	delete(internal.InstanceMap, c.InstanceId)
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
