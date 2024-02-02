package validator

import (
	"errors"
	"github.com/go-playground/validator/v10"
	"reflect"
)

// GetValidMsg Get the 'msg' tag of the field if error exists
func GetValidMsg(err error, obj any) string {
	getObj := reflect.TypeOf(obj)
	var errs validator.ValidationErrors
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

func IsIdValid(id int64) bool {
	return id > 0
}
