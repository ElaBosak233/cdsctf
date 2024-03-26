package validator

import (
	"errors"
	"github.com/duke-git/lancet/v2/validator"
	v10 "github.com/go-playground/validator/v10"
	"reflect"
)

func IsASCII(fl v10.FieldLevel) bool {
	return validator.IsASCII(fl.Field().String())
}

// GetValidMsg Get the 'msg' tag of the field if error exists
func GetValidMsg(err error, obj any) string {
	getObj := reflect.TypeOf(obj)
	var errs v10.ValidationErrors
	if errors.As(err, &errs) {
		for _, e := range errs {
			if f, exits := getObj.Elem().FieldByName(e.Field()); exits {
				msg := f.Tag.Get("msg")
				return msg
			}
		}
	}
	return err.Error()
}
