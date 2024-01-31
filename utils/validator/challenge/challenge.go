package challenge

func IsIdValid(id int64) bool {
	return id > 0
}

func IsIdArrayValid(ids []int64) bool {
	for _, id := range ids {
		if !IsIdValid(id) {
			return false
		}
	}
	return len(ids) > 0
}

func IsTitleStringValid(title string) bool {
	return title != ""
}

func IsCategoryStringValid(category string) bool {
	return category != ""
}

func IsDifficultyIntValid(difficulty int64) bool {
	return difficulty >= 1 && difficulty <= 5
}

func IsDynamicIntValid(isDynamic int) bool {
	return isDynamic == 1 || isDynamic == 0
}

func IsPracticableIntValid(isPracticable int) bool {
	return isPracticable == 1 || isPracticable == 0
}
