package service

var InstanceMap = make(map[string]map[string]interface{})

type InstanceService interface {
	Create(challengeId string) (instanceId string, entry string)
	Status(id string) (status string, entry string, error error)
	Renew(id string) error
	Remove(id string) error
}
