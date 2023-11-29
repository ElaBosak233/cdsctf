package service

var InstanceMap = make(map[string]interface{})

type InstanceService interface {
	Create(challengeId string) (instanceId string)
	Status(id string) (status string, error error)
	Renew(id string) error
	Remove(id string) error
}
