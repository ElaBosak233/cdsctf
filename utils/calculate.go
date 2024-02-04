package utils

import (
	"math"
)

// CalculateChallengePts Calculate the dynamic points of game.
// "S" is the maximum points，
// "R" is the minimum points.
// "d" is the degree of difficulty of the challenge.
// "x" is the number of submissions.
func CalculateChallengePts(S, R, d int64, x int) int64 {
	ratio := float64(R) / float64(S)
	result := int64(math.Floor(float64(S) * (ratio + (1-ratio)*math.Exp((1-float64(x))/float64(d)))))
	return min(result, S)
}
