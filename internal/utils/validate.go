package utils

import (
	"errors"
	"fmt"
	"github.com/go-playground/validator/v10"
	"reflect"
	"strconv"
)

// GetValidMsg 获取错误信息
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

// ParseIntParam 获取 int 参数
func ParseIntParam(queryValue string, defaultValue int) int {
	if value, err := strconv.Atoi(queryValue); err == nil {
		return value
	}
	return defaultValue
}

// MapStringsToInts 字符串数组转 int64 数组
func MapStringsToInts(strArray []string) []int64 {
	intArray := make([]int64, len(strArray))
	for i, str := range strArray {
		num, err := strconv.Atoi(str)
		if err != nil {
			fmt.Printf("Error converting string to int: %v\n", err)
			return nil
		}
		intArray[i] = int64(num)
	}
	return intArray
}
