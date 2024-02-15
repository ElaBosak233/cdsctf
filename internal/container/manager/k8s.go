package manager

import (
	"context"
	"errors"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/elabosak233/pgshub/internal/config"
	"github.com/elabosak233/pgshub/internal/container/provider"
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/google/uuid"
	"go.uber.org/zap"
	corev1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/api/resource"
	v1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"time"
)

var (
	namespace string
)

type K8sManager struct {
	PodID      uint
	RespID     string
	Images     []*model.Image
	Flag       model.Flag
	Instances  []*model.Instance
	Inspect    corev1.Pod
	Duration   time.Duration
	CancelCtx  context.Context
	CancelFunc context.CancelFunc
}

func NewK8sManager(podID uint, images []*model.Image, flag model.Flag, duration time.Duration) *K8sManager {
	namespace = config.AppCfg().Container.K8s.Namespace
	return &K8sManager{
		PodID:    podID,
		Images:   images,
		Duration: duration,
		Flag:     flag,
	}
}

func (c *K8sManager) Setup() (instances []*model.Instance, err error) {
	var containers []corev1.Container
	var imageMap = make(map[string]uint)
	for _, image := range c.Images {
		var ports []corev1.ContainerPort
		for _, port := range image.Ports {
			// Don't set HostPort because it should be decided by Kubernetes
			ports = append(ports, corev1.ContainerPort{
				ContainerPort: int32(port.Value),
			})
		}

		var envs []corev1.EnvVar
		for _, env := range image.Envs {
			envs = append(envs, corev1.EnvVar{Name: env.Key, Value: env.Value})
		}
		// Add the flag information to the environment variables
		envs = append(envs, corev1.EnvVar{Name: c.Flag.Env, Value: c.Flag.Value})
		uid := uuid.NewString()
		imageMap[uid] = image.ID
		containers = append(containers, corev1.Container{
			Name:  uid,
			Image: image.Name,
			Env:   envs,
			Ports: ports,
			Resources: corev1.ResourceRequirements{
				Limits: corev1.ResourceList{
					corev1.ResourceCPU:    resource.MustParse(fmt.Sprintf("%f", image.CPULimit)),
					corev1.ResourceMemory: resource.MustParse(fmt.Sprintf("%dMi", image.MemoryLimit)),
				},
			},
		})
	}

	pod := &corev1.Pod{
		ObjectMeta: v1.ObjectMeta{
			GenerateName: uuid.NewString(),
			Labels: map[string]string{
				"app": "pgshub",
			},
		},
		Spec: corev1.PodSpec{
			Containers: containers,
		},
	}

	pod, err = provider.K8sCli().CoreV1().Pods(namespace).Create(context.Background(), pod, v1.CreateOptions{})
	if err != nil {
		zap.L().Error(fmt.Sprintf("[%s] Unable to create pod.", color.InCyan("K8S")), zap.Error(err))
	}
	c.RespID = pod.Name
	c.Inspect = *pod
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())

	// Get the created pod's information
	createdPod, err := provider.K8sCli().CoreV1().Pods(namespace).Get(context.Background(), c.RespID, v1.GetOptions{})
	if err != nil {
		return nil, err
	}

	// Extract the assigned ports from the pod's information
	for _, container := range createdPod.Spec.Containers {
		instance := model.Instance{
			PodID:   c.PodID,
			ImageID: imageMap[container.Name],
		}
		for _, port := range container.Ports {
			nat := &model.Nat{
				SrcPort: int(port.ContainerPort),
				DstPort: int(port.HostPort),
			}
			instance.Nats = append(instance.Nats, nat)
		}
		instances = append(instances, &instance)
	}

	return instances, err
}

func (c *K8sManager) GetContainerStatus() (status string, err error) {
	if c.RespID == "" {
		return "", errors.New("pod not created or initialization failed")
	}
	pod, err := provider.K8sCli().CoreV1().Pods(namespace).Get(context.Background(), c.RespID, v1.GetOptions{})
	if err != nil {
		return "removed", err
	}
	return string(pod.Status.Phase), err
}

func (c *K8sManager) RemoveAfterDuration(ctx context.Context) (success bool) {
	select {
	case <-time.After(c.Duration):
		c.Remove()
		return true
	case <-ctx.Done():
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
	c.Duration = duration
	c.CancelCtx, c.CancelFunc = context.WithCancel(context.Background())
	go c.RemoveAfterDuration(c.CancelCtx)
	zap.L().Warn(
		fmt.Sprintf("[%s] Pod %d (RespID %s) successfully renewed.",
			color.InCyan("K8S"),
			c.PodID,
			c.RespID,
		),
	)
}
