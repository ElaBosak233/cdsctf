package utils

import (
	"errors"
	"github.com/go-playground/validator/v10"
	"reflect"
	"strconv"
)

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

func ParseIntParam(queryValue string, defaultValue int) int {
	if value, err := strconv.Atoi(queryValue); err == nil {
		return value
	}
	return defaultValue
}
