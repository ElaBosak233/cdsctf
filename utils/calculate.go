package utils

import (
	"math"
)

// CalculateChallengePts 计算 CTF 比赛动态分值
// S 是最大分值，R 是最小分值，d 是题目的难度系数（1~5），x 是本题被解出来的次数
func CalculateChallengePts(S, R, d int64, x int) int64 {
	ratio := float64(R) / float64(S)
	result := int64(math.Floor(float64(S) * (ratio + (1-ratio)*math.Exp((1-float64(x))/float64(d)))))
	return min(result, S)
}
