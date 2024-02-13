package challenge

// IsIdValid Judge whether id is valid
func IsIdValid(id int64) bool {
	return id > 0
}

// IsIdArrayValid Judge whether id array is valid
func IsIdArrayValid(ids []int64) bool {
	for _, id := range ids {
		if !IsIdValid(id) {
			return false
		}
	}
	return len(ids) > 0
}

// IsTitleStringValid Judge whether 'title' value is valid
func IsTitleStringValid(title string) bool {
	return title != ""
}

// IsCategoryStringValid Judge whether 'category' value is valid
func IsCategoryStringValid(category string) bool {
	return category != ""
}

// IsDifficultyIntValid Judge whether 'difficulty' value is valid
func IsDifficultyIntValid(difficulty int64) bool {
	return difficulty >= 1 && difficulty <= 5
}

// IsDynamicIntValid Judge whether 'is_dynamic' value is valid
func IsDynamicIntValid(isDynamic int) bool {
	return isDynamic == 1 || isDynamic == 0
}

// IsPracticableIntValid Judge whether 'is_practicable' value is valid
func IsPracticableIntValid(isPracticable int) bool {
	return isPracticable == 1 || isPracticable == 0
}
