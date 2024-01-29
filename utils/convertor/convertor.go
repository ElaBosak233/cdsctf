package convertor

import "github.com/duke-git/lancet/v2/convertor"

func ToInt64D(v string, d int64) int64 {
	result, err := convertor.ToInt(v)
	if err != nil {
		return d
	}
	return result
}

func ToIntD(v string, d int) int {
	return int(ToInt64D(v, int64(d)))
}
