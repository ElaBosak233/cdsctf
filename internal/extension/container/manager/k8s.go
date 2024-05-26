package manager

import (
	"context"
	"errors"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/elabosak233/cloudsdale/internal/extension/config"
	"github.com/elabosak233/cloudsdale/internal/extension/container/provider"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/utils/generator"
	"go.uber.org/zap"
	corev1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/api/resource"
	v1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/util/intstr"
	"time"
)

var (
	namespace string
)

type K8sManager struct {
	challenge model.Challenge
	flag      model.Flag
	duration  time.Duration

	PodID      uint
	RespID     string
	Nats       []*model.Nat
	Inspect    corev1.Pod
	CancelCtx  context.Context
	CancelFunc context.CancelFunc
}

func NewK8sManager(challenge model.Challenge, flag model.Flag, duration time.Duration) IContainerManager {
	namespace = config.AppCfg().Container.K8s.NameSpace
	return &K8sManager{
		challenge: challenge,
		duration:  duration,
		flag:      flag,
	}
}

func (c *K8sManager) Setup() (nats []*model.Nat, err error) {
	var containers []corev1.Container
	var ports []corev1.ContainerPort
	for _, port := range c.challenge.Ports {
		// Don't set HostPort because it should be decided by Kubernetes
		ports = append(ports, corev1.ContainerPort{
			ContainerPort: int32(port.Value),
		})
	}

	var envs []corev1.EnvVar
	for _, env := range c.challenge.Envs {
		envs = append(envs, corev1.EnvVar{Name: env.Key, Value: env.Value})
	}
	// Add the flag information to the environment variables
	envs = append(envs, corev1.EnvVar{Name: c.flag.Env, Value: c.flag.Value})
	uid := generator.HyphenlessUUID()
	containers = append(containers, corev1.Container{
		Name:  uid,
		Image: c.challenge.ImageName,
		Env:   envs,
		Ports: ports,
		Resources: corev1.ResourceRequirements{
			Limits: corev1.ResourceList{
				corev1.ResourceCPU:    resource.MustParse(fmt.Sprintf("%d", c.challenge.CPULimit)),
				corev1.ResourceMemory: resource.MustParse(fmt.Sprintf("%dMi", c.challenge.MemoryLimit)),
			},
		},
	})

	pod := &corev1.Pod{
		ObjectMeta: v1.ObjectMeta{
			GenerateName: fmt.Sprintf("cloudsdale-%s", uid),
			Labels: map[string]string{
				"app": "cloudsdale",
			},
		},
		Spec: corev1.PodSpec{
			Containers: containers,
		},
	}

	pod, err = provider.K8sCli().CoreV1().Pods(namespace).Create(context.Background(), pod, v1.CreateOptions{})
	if err != nil {
		zap.L().Error(fmt.Sprintf("[%s] Unable to create pod.", color.InCyan("K8S")), zap.Error(err))
		return nil, err
	}
	c.RespID = pod.Name
	c.Inspect = *pod
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())

	// Create a NodePort service to expose the pod
	service := &corev1.Service{
		ObjectMeta: v1.ObjectMeta{
			Name:      fmt.Sprintf("cloudsdale-%s", uid),
			Namespace: namespace,
			Labels: map[string]string{
				"app": "cloudsdale",
			},
		},
		Spec: corev1.ServiceSpec{
			Type: corev1.ServiceTypeNodePort,
			Selector: map[string]string{
				"app": "cloudsdale",
			},
			Ports: []corev1.ServicePort{},
		},
	}

	for _, port := range c.challenge.Ports {
		servicePort := corev1.ServicePort{
			Port:       int32(port.Value),
			TargetPort: intstr.FromInt32(int32(port.Value)),
		}
		service.Spec.Ports = append(service.Spec.Ports, servicePort)
	}

	service, err = provider.K8sCli().CoreV1().Services(namespace).Create(context.Background(), service, v1.CreateOptions{})
	if err != nil {
		zap.L().Error(fmt.Sprintf("[%s] Unable to create service.", color.InCyan("K8S")), zap.Error(err))
		return nil, err
	}

	// Get the created service's information
	createdService, err := provider.K8sCli().CoreV1().Services(namespace).Get(context.Background(), service.Name, v1.GetOptions{})
	if err != nil {
		return nil, err
	}

	// Extract the assigned NodePorts from the service's information
	for _, servicePort := range createdService.Spec.Ports {
		nat := &model.Nat{
			SrcPort: int(servicePort.Port),
			DstPort: int(servicePort.NodePort),
			Entry: fmt.Sprintf(
				"%s:%d",
				config.AppCfg().Container.K8s.Entry,
				servicePort.NodePort,
			),
		}
		nats = append(nats, nat)
	}

	return nats, nil
}

func (c *K8sManager) SetPodID(podID uint) {
	c.PodID = podID
}

func (c *K8sManager) Duration() (duration time.Duration) {
	return c.duration
}

func (c *K8sManager) Status() (status string, err error) {
	if c.RespID == "" {
		return "", errors.New("pod not created or initialization failed")
	}
	pod, err := provider.K8sCli().CoreV1().Pods(namespace).Get(context.Background(), c.RespID, v1.GetOptions{})
	if err != nil {
		return "removed", err
	}
	return string(pod.Status.Phase), err
}

func (c *K8sManager) RemoveAfterDuration() (success bool) {
	select {
	case <-time.After(c.duration):
		c.Remove()
		return true
	case <-c.CancelCtx.Done():
		zap.L().Warn(fmt.Sprintf("[%s] Pod %d (RespID %s)'s removal plan has been cancelled.", color.InCyan("K8S"), c.PodID, c.RespID))
		return false
	}
}

func (c *K8sManager) Remove() {
	go func() {
		_ = provider.K8sCli().CoreV1().Pods(namespace).Delete(context.Background(), c.RespID, v1.DeleteOptions{})
	}()
}

func (c *K8sManager) Renew(duration time.Duration) {
	if c.CancelFunc != nil {
		c.CancelFunc() // Calling the cancel function
	}
	c.duration = duration
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration()
	zap.L().Warn(
		fmt.Sprintf("[%s] Pod %d (RespID %s) successfully renewed.",
			color.InCyan("K8S"),
			c.PodID,
			c.RespID,
		),
	)
}
