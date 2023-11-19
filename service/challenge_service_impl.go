package service

import (
	model "github.com/elabosak233/pgshub/model/data"
	req "github.com/elabosak233/pgshub/model/request/challenge"
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/utils"
	"github.com/go-playground/validator/v10"
	"github.com/google/uuid"
	"reflect"
)

type ChallengeServiceImpl struct {
	ChallengeRepository repository.ChallengeRepository
	Validate            *validator.Validate
}

func NewChallengeServiceImpl(appRepository repository.AppRepository) ChallengeService {
	return &ChallengeServiceImpl{
		ChallengeRepository: appRepository.ChallengeRepository,
		Validate:            validator.New(),
	}
}

// Create implements UserService
func (t *ChallengeServiceImpl) Create(req req.CreateChallengeRequest) error {
	err := t.Validate.Struct(req)
	if err != nil {
		return err
	}
	challengeModel := model.Challenge{
		Id:          uuid.NewString(),
		Title:       req.Title,
		Description: req.Description,
		UploaderId:  req.UploaderId,
	}
	err = t.ChallengeRepository.Insert(challengeModel)
	return err
}

// Update implements UserService
func (t *ChallengeServiceImpl) Update(req map[string]interface{}) error {
	challengeData, err := t.ChallengeRepository.FindById(req["id"].(string))
	if err != nil {
		return err
	}
	delete(req, "created_at")
	delete(req, "updated_at")
	delete(req, "uploader_id")
	updateFields(&challengeData, req)
	err = t.ChallengeRepository.Update(challengeData)
	return err
}

// Delete implements UserService
func (t *ChallengeServiceImpl) Delete(id string) error {
	err := t.ChallengeRepository.Delete(id)
	return err
}

// FindAll implements UserService
func (t *ChallengeServiceImpl) FindAll() []model.Challenge {
	result := t.ChallengeRepository.FindAll()
	var challenges []model.Challenge
	for _, value := range result {
		challenge := value
		challenges = append(challenges, challenge)
	}

	return challenges
}

// FindById implements UserService
func (t *ChallengeServiceImpl) FindById(id string) model.Challenge {
	challengeData, _ := t.ChallengeRepository.FindById(id)
	return challengeData
}

func updateFields(challengeData *model.Challenge, req map[string]interface{}) {
	challengeType := reflect.TypeOf(challengeData).Elem()
	challengeValue := reflect.ValueOf(challengeData).Elem()
	for i := 0; i < challengeType.NumField(); i++ {
		challengeField := challengeType.Field(i)
		fieldName := challengeField.Tag.Get("json")
		reqFieldValue, ok := req[fieldName]
		if !ok {
			continue
		}
		field := challengeValue.FieldByName(challengeField.Name)
		if field.IsValid() {
			if field.Kind() == reflect.Ptr {
				field = field.Elem()
			}
			reqFieldValue, err := convertType(reqFieldValue, field.Type())
			if err != nil {
				utils.Logger.Error("类型转换错误")
				continue
			}
			field.Set(reflect.ValueOf(reqFieldValue))
		}
	}
}

func convertType(value interface{}, targetType reflect.Type) (interface{}, error) {
	switch targetType.Kind() {
	case reflect.Int:
		return int(value.(float64)), nil
	default:
		return value, nil
	}
}
