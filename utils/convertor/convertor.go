package convertor

import (
	"github.com/duke-git/lancet/v2/convertor"
	"strconv"
)

func TrueP() *bool {
	i := true
	return &i
}

func FalseP() *bool {
	i := false
	return &i
}

func ToInt64D(v string, d int64) int64 {
	result, err := convertor.ToInt(v)
	if err != nil {
		return d
	}
	return result
}

func ToInt64P(v string) *int64 {
	result, err := convertor.ToInt(v)
	if err != nil {
		return nil
	}
	return &result
}

func ToIntD(v string, d int) int {
	return int(ToInt64D(v, int64(d)))
}

func ToIntP(v string) *int {
	result64, err := convertor.ToInt(v)
	if err != nil {
		return nil
	}
	result := int(result64)
	return &result
}

func ToBoolD(v string, d bool) bool {
	result, err := convertor.ToBool(v)
	if err != nil {
		return d
	}
	return result
}

func ToBoolP(v string) *bool {
	result, err := convertor.ToBool(v)
	if err != nil {
		return nil
	}
	return &result
}

func ToInt64SliceD(strSlice []string, d []int64) []int64 {
	int64Slice := make([]int64, len(strSlice))
	for i, str := range strSlice {
		num, err := strconv.Atoi(str)
		if err != nil {
			return d
		}
		int64Slice[i] = int64(num)
	}
	return int64Slice
}
