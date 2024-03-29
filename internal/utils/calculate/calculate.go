package calculate

import (
	"math"
)

// ChallengePts Calculate the dynamic points of game.
// "S" is the maximum points，
// "R" is the minimum points.
// "d" is the degree of difficulty of the challenge.
// "x" is the number of submissions.
func ChallengePts(S, R, d int64, x int) int64 {
	ratio := float64(R) / float64(S)
	result := int64(math.Floor(float64(S) * (ratio + (1-ratio)*math.Exp((1-float64(x))/float64(d)))))
	return min(result, S)
}

func GameChallengePts(S, R, d int64, x int, firstBloodRewardRatio, secondBloodRewardRatio, thirdBloodRewardRatio float64) int64 {
	pts := ChallengePts(S, R, d, x)
	switch x {
	case 0:
		pts = int64(math.Floor(((firstBloodRewardRatio / 100) + 1) * float64(pts)))
	case 1:
		pts = int64(math.Floor(((secondBloodRewardRatio / 100) + 1) * float64(pts)))
	case 2:
		pts = int64(math.Floor(((thirdBloodRewardRatio / 100) + 1) * float64(pts)))
	}
	return pts
}
