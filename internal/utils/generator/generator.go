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

func GenerateWebSocketKey() (string, error) {
	key := make([]byte, 16)
	_, err := rand.Read(key)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(key), nil
}
