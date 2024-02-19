package generator

import (
	"crypto/rand"
	"encoding/base64"
	"github.com/google/uuid"
	"strings"
)

func GenerateFlag(flagFmt string) (flag string) {
	flag = strings.Replace(flagFmt, "[UUID]", uuid.NewString(), -1)
	return flag
}

func GenerateWebSocketKey() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return base64.StdEncoding.EncodeToString(b)
}
