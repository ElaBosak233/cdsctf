package provider

import (
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/embed"
	"go.uber.org/zap"
	"io"
	"io/fs"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/tools/clientcmd"
	"os"
)

var (
	// k8sCli Store Kubernetes client pointers
	k8sCli *kubernetes.Clientset
)

func K8sCli() *kubernetes.Clientset {
	return k8sCli
}

func InitK8sProvider() {
	kubeconfig := config.AppCfg().Container.K8s.Path.Config
	checkK8sConfig(kubeconfig)
	cfg, err := clientcmd.BuildConfigFromFlags("", kubeconfig)
	if err != nil {
		zap.L().Fatal("Kubernetes config initialization failed.", zap.Error(err))
	}
	k8sClient, err := kubernetes.NewForConfig(cfg)
	if err != nil {
		zap.L().Fatal("Kubernetes client initialization failed.", zap.Error(err))
	}
	k8sCli = k8sClient
	serverVersion, err := k8sClient.Discovery().ServerVersion()
	if err != nil {
		zap.L().Fatal("Kubernetes server connection failure.", zap.Error(err))
	}
	zap.L().Info(fmt.Sprintf("Kubernetes remote server connection successful, server version %s", color.InCyan(serverVersion)))
}

func checkK8sConfig(kubeconfig string) {
	if _, err := os.Stat(kubeconfig); err != nil {
		if kubeconfig == "./k8s-config.yml" {
			zap.L().Warn("No Kubernetes configuration file found, default configuration file will be created.")
			// Read default configuration from embed
			defaultConfig, _err := embed.FS.Open("k8s-config.yml")
			if _err != nil {
				zap.L().Error("Unable to read default configuration file.")
				return
			}
			defer func(defaultConfig fs.File) {
				_ = defaultConfig.Close()
			}(defaultConfig)

			// Create config file in current directory
			dstConfig, _err := os.Create(kubeconfig)
			defer func(dstConfig *os.File) {
				_ = dstConfig.Close()
			}(dstConfig)

			if _, _err = io.Copy(dstConfig, defaultConfig); _err != nil {
				zap.L().Fatal("Unable to create default configuration file.")
			}
			zap.L().Info("The default configuration file has been generated.")
		} else {
			zap.L().Fatal("Kubernetes configuration file not found.")
		}
	}
}
