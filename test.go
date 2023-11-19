package main

import (
	"fmt"
	"github.com/docker/docker/client"
	"github.com/elabosak233/pgshub/container"
	"github.com/elabosak233/pgshub/utils"
	"time"
)

func main() {
	// 初始化 Logger
	utils.InitLogger()
	// 初始化配置文件
	utils.LoadConfig()
	// 连接到 Docker 客户端，这里使用默认的本地 Docker 客户端
	cli, _ := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
	// 创建一个 DockerContainer 实例
	ctn := container.NewContainer(cli, "my_ctf_image", 80, "DASCTF{怎么都对}", "FLAG", 1024*1024*1024, 3*time.Second)
	// 测试 Setup 方法
	_ = ctn.Setup()
	// 测试 GetContainerStatus 方法
	fmt.Println(ctn.RespId)
	status, _ := ctn.GetContainerStatus()
	fmt.Println(status)
	time.Sleep(30 * time.Second)
	status, _ = ctn.GetContainerStatus()
	fmt.Println(status)
}
