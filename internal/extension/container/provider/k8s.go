package provider

import (
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"go.uber.org/zap"
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
	k8sConfig := config.AppCfg().Container.K8s.Config
	checkK8sConfig(k8sConfig)
	cfg, err := clientcmd.BuildConfigFromFlags("", k8sConfig)
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

func checkK8sConfig(k8sConfig string) {
	if _, err := os.Stat(k8sConfig); err != nil {
		zap.L().Fatal("Kubernetes configuration file not found.")
	}
}
