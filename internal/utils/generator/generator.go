package generator

import (
	"github.com/google/uuid"
	"strings"
)

func GenerateFlag(flagFmt string) (flag string) {
	flag = strings.Replace(flagFmt, "[UUID]", uuid.NewString(), -1)
	return flag
}
